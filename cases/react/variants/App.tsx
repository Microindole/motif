export function App() {
  return (
    <section className="m-surface f-stack dark:m-elevation-1 ui-pad-xl ui-radius-lg ui-density-comfortable">
      <p className="ui-text-xs">Dense helper copy</p>
      <button className="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill ui-density-compact">
        React variants
      </button>
      <div className="ui-stack-inline ui-gap-xl">
        <input type="checkbox" className="f-checkbox" />
        <input type="radio" className="m-radio" />
        <button className="m-switch">Toggle</button>
      </div>
      <div className="ui-stack-center ui-text-xs">Preset + ui layer</div>
    </section>
  );
}
