import test from 'node:test';
import assert from 'node:assert/strict';
import { compileSources } from '../../src/core.js';

// Batch tests track the staged component-matrix rollout; each block represents a vertical
// slice that should keep rendering once preset rules move between shared/foundation/shell layers.
test('compileSources renders fourth batch shell and identity semantics', () => {
  const result = compileSources([
    {
      path: '/demo/Shell.tsx',
      content: `
        <aside class="f-drawer"></aside>
        <section class="m-toast"></section>
        <button class="m-segmented-button"></button>
        <input class="f-search-field" />
        <a class="m-breadcrumb-item"></a>
        <div class="f-avatar"></div>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-drawer \{/);
  assert.match(result.stylesheet, /\.m-toast \{/);
  assert.match(result.stylesheet, /\.m-segmented-button \{/);
  assert.match(result.stylesheet, /\.f-search-field \{/);
  assert.match(result.stylesheet, /\.m-breadcrumb-item \{/);
  assert.match(result.stylesheet, /\.f-avatar \{/);
});

test('compileSources renders fifth batch loading and empty semantics', () => {
  const result = compileSources([
    {
      path: '/demo/Feedback.tsx',
      content: `
        <div class="f-progress"></div>
        <div class="m-spinner"></div>
        <div class="f-skeleton"></div>
        <section class="m-empty-state"></section>
        <aside class="f-sheet"></aside>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-progress \{/);
  assert.match(result.stylesheet, /\.m-spinner \{/);
  assert.match(result.stylesheet, /\.f-skeleton \{/);
  assert.match(result.stylesheet, /\.m-empty-state \{/);
  assert.match(result.stylesheet, /\.f-sheet \{/);
  assert.match(result.stylesheet, /@keyframes motif-spin/);
  assert.match(result.stylesheet, /@keyframes motif-shimmer/);
});

test('compileSources renders sixth batch structured data semantics', () => {
  const result = compileSources([
    {
      path: '/demo/Data.tsx',
      content: `
        <section class="f-accordion-item"></section>
        <div class="m-table-row"></div>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-accordion-item \{/);
  assert.match(result.stylesheet, /\.m-table-row \{/);
  assert.match(result.stylesheet, /grid-template-columns: minmax\(0, 2fr\) minmax\(0, 1fr\) auto;/);
});

test('compileSources renders seventh batch identity and navigation groups', () => {
  const result = compileSources([
    {
      path: '/demo/Identity.tsx',
      content: `
        <nav class="f-breadcrumb"></nav>
        <section class="m-persona"></section>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-breadcrumb \{/);
  assert.match(result.stylesheet, /\.m-persona \{/);
  assert.match(result.stylesheet, /flex-wrap: wrap;/);
});

test('compileSources renders eighth batch structured states', () => {
  const result = compileSources([
    {
      path: '/demo/States.tsx',
      content: `
        <section class="f-accordion-item-open"></section>
        <div class="m-table-row-selected"></div>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-accordion-item-open \{/);
  assert.match(result.stylesheet, /\.m-table-row-selected \{/);
  assert.match(result.stylesheet, /border-color: #1a73e8;/);
});

test('compileSources renders ninth batch chip and sheet variants', () => {
  const result = compileSources([
    {
      path: '/demo/Layout.tsx',
      content: `
        <span class="f-chip"></span>
        <aside class="m-sheet-side"></aside>
        <aside class="f-sheet-bottom"></aside>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-chip \{/);
  assert.match(result.stylesheet, /\.m-sheet-side \{/);
  assert.match(result.stylesheet, /\.f-sheet-bottom \{/);
  assert.match(result.stylesheet, /max-width: 36rem;/);
});

test('compileSources renders tenth batch table container and tag semantics', () => {
  const result = compileSources([
    {
      path: '/demo/DataGroup.tsx',
      content: `
        <div class="f-table"></div>
        <span class="m-tag"></span>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-table \{/);
  assert.match(result.stylesheet, /\.m-tag \{/);
  assert.match(result.stylesheet, /min-height: 1.5rem;/);
});

test('compileSources renders eleventh batch table and accordion headers', () => {
  const result = compileSources([
    {
      path: '/demo/Headers.tsx',
      content: `
        <div class="f-table-header"></div>
        <div class="m-accordion-header"></div>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-table-header \{/);
  assert.match(result.stylesheet, /\.m-accordion-header \{/);
  assert.match(result.stylesheet, /justify-content: space-between;/);
});

test('compileSources renders twelfth batch table cells and accordion content', () => {
  const result = compileSources([
    {
      path: '/demo/Cells.tsx',
      content: `
        <div class="f-table-cell"></div>
        <div class="m-accordion-content"></div>
      `,
    },
  ]);

  assert.match(result.stylesheet, /\.f-table-cell \{/);
  assert.match(result.stylesheet, /\.m-accordion-content \{/);
  assert.match(result.stylesheet, /gap: 0.5rem;/);
});
