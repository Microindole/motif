export interface MotifSourceFile {
  path: string;
  content: string;
}

export interface MotifCompileResult {
  stylesheet: string;
}

export interface MotifCompileAdapter {
  compileSources(sources: MotifSourceFile[]): MotifCompileResult | Promise<MotifCompileResult>;
}
