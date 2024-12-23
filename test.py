import os
import openai

# optional; defaults to `os.environ['OPENAI_API_KEY']`
openai.api_key = "sk-rDdHLOO37FL7qdzX9857512a5eAf43F7Ad0cA27f1b554f65"

# all client options can be configured just like the `OpenAI` instantiation counterpart
openai.base_url = "https://free.v36.cm/v1/"
openai.default_headers = {"x-foo": "true"}

completion = openai.chat.completions.create(
    model="gpt-3.5-turbo",
    messages=[
        {
            "role": "user",
            "content": "Hello world!",
        },
    ],
)
print(completion.choices[0].message.content)