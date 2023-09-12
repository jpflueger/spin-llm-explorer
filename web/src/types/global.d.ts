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
}
