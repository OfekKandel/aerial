import json
import tomllib
import subprocess
from openai import OpenAI
from openai.types.chat.chat_completion_message import ChatCompletionMessage

MODEL = "gpt-4o"


# NOTE: The config is in config_ai.toml because aerial-utils uses config.toml
# TODO: Fix config.toml colision
def get_config():
    with open("config_ai.toml", "rb") as file:
        return tomllib.load(file)


def get_tools_spec():
    with open("tools-spec.json", "rb") as file:
        return json.load(file)


def add_chat(prompt: str, messages: list[dict]):
    messages.append({"role": "user", "content": prompt})


def make_completion(client: OpenAI, messages: list[dict], tools: list[dict]):
    completion = client.chat.completions.create(
        model=MODEL, messages=messages, tools=tools
    )
    completion = completion.choices[0].message
    messages.append(completion)
    return completion


def handle_completion(
    message: ChatCompletionMessage,
    client: OpenAI,
    messages: list[dict],
    tools: list[dict],
) -> ChatCompletionMessage:
    if not message.tool_calls:
        return message

    for tool_call in message.tool_calls:
        # TODO: Add logger for debugging
        tool_response = handle_function(
            tool_call.function.name, json.loads(tool_call.function.arguments), tools
        )
        messages.append(
            {
                "tool_call_id": tool_call.id,
                "role": "tool",
                "name": tool_call.function.name,
                "content": tool_response,
            }
        )
    completion = make_completion(client, messages, tools)
    return handle_completion(completion, client, messages, tools)


def parse_arguments(arguments: dict, tools: list[dict], func_name: str) -> list[str]:
    tool = next((tool for tool in tools if tool["function"]["name"] == func_name))
    pos_args = []
    other_args = []
    for name, data in tool["function"]["parameters"]["properties"].items():
        if "index" in data:
            pos_args.insert(data["index"], arguments[name])
        elif name in arguments:
            other_args.append(f"--{name}")
            other_args.append(str(arguments[name]))
    print(f"{func_name} {pos_args + other_args}")
    return pos_args + other_args


def handle_function(name: str, arguments: dict, tools: list[dict]):
    commands = (
        ["./aerial-utils"] + name.split("_") + parse_arguments(arguments, tools, name)
    )
    process = subprocess.run(commands, capture_output=True, text=True, cwd=".")
    # TODO: This is a weird approach because the aerial-utils error is in stderr and not stdout
    return process.stdout + process.stderr


def main():
    config = get_config()
    client = OpenAI(api_key=config["openai"]["api_key"])
    tools = get_tools_spec()

    messages = [{"role": "system", "content": config["prompt"]}]

    while True:
        prompt = input("<User> ")
        add_chat(prompt, messages)
        completion = make_completion(client, messages, tools)
        completion = handle_completion(completion, client, messages, tools)
        print(f"<AI> {completion.content}")


if __name__ == "__main__":
    main()
