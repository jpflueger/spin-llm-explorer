use crate::model::{Message, MessageRole};

pub trait PromptFormatter {
    fn format(&self, messages: &[Message]) -> String;
}

#[derive(Default)]
pub struct Llama2PromptFormatter {
}

impl Llama2PromptFormatter {
    pub(self) fn format_usr(content: &str) -> String {
        format!("{} [/INST]", content)
    }

    pub(self) fn format_asst(content: &str) -> String {
        format!("{} </s><s>[INST]", content)
    }

    pub(self) fn format_sys(content: &str) -> String {
        format!("<<SYS>>\n{}\n<</SYS>>", content)
    }
}

impl PromptFormatter for Llama2PromptFormatter {
  /*
  based on -> https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML/discussions/3

  example single prompt:
    <s>[INST] <<SYS>>
    You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

    If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.
    <</SYS>>

    <prompt> [/INST]

  example chat prompt:
    <s>[INST] <<SYS>>
    You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

    If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.
    <</SYS>>

    <prompt> [/INST] <answer> </s><s>[INST] <prompt-second> [/INST]
  */

    ///
    fn format(&self, messages: &[Message]) -> String {
        let mut prompt = String::new();
        prompt.push_str("<s>[INST] ");

        for msg in messages {
          let formatted = match msg.role {
            MessageRole::System => Self::format_sys(&msg.content),
            MessageRole::User => Self::format_usr(&msg.content),
            MessageRole::Assistant => Self::format_asst(&msg.content),
          };
          prompt.push_str(&formatted);
          prompt.push('\n');
          prompt.push('\n');
        }

        // remove the last two newlines
        prompt.pop();
        prompt.pop();

        prompt
    }
}
