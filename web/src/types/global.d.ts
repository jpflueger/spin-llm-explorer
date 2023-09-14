export { }

declare global {
  interface App {
    name: string;
    description: string;
    system_prompt: string;
    model: {
      name: string;
      max_tokens: number;
      temperature: number;
      repeat_penalty: number;
      repeat_penalty_last_n_tokens: number;
      top_k: number;
      top_p: number;
    }
  }

  interface CompletionMessage {
    role: string;
    content: string;
  }

  interface CompletionUsage {
    prompt_token_count: number;
    generated_token_count: number;
  }

  interface Completion {
    messages: CompletionMessage[];
    usage: CompletionUsage | null;
  }
}
