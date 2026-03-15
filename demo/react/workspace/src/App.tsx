import { FluentWorkspaceSection } from "./sections/FluentWorkspaceSection";
import { MaterialWorkspaceSection } from "./sections/MaterialWorkspaceSection";

const shellStyle = {
  minHeight: "100vh",
  display: "grid",
  gridTemplateColumns: "repeat(auto-fit, minmax(320px, 1fr))",
  gap: "24px",
  padding: "32px",
  background: "linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%)",
} as const;

export function App() {
  return (
    <main style={shellStyle}>
      <FluentWorkspaceSection />
      <MaterialWorkspaceSection />
    </main>
  );
}
