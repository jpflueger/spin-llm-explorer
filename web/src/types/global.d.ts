export { }

declare global {
  interface App {
    created_at: string;
    updated_at: string;
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

  interface AppCompletion {
    created_at: string;
    user_prompt: string;
    app_snapshot: App;
    completion: Completion;
  }
}
