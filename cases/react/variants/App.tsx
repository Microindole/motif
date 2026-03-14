export function App() {
  return (
    <section className="m-surface f-stack dark:m-elevation-1 ui-pad-lg ui-radius-lg">
      <button className="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill">
        React variants
      </button>
      <input type="checkbox" className="f-checkbox" />
      <input type="radio" className="m-radio" />
      <button className="m-switch">Toggle</button>
    </section>
  );
}
