# html/semantics/sections/headingoffset-and-headingreset.html

Counts:
- errors: 0
- warnings: 86
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/sections/headingoffset-and-headingreset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<link rel="help" href="https://github.com/whatwg/html/pull/11086" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<div headingoffset="1" title="container headingoffset=1">
  <!-- h1s are now h2s and so on -->
  <h1 data-expected-offset="2"><!-- Level 2, h1 + 1 = 2 --></h1>
  <h2 data-expected-offset="3"><!-- Level 3, h2 + 1 = 3 --></h2>
  <h3 data-expected-offset="4"><!-- Level 4, h3 + 1 = 4 --></h3>
  <div headingoffset="2" title="container headingoffset=2">
    <!-- h1s are now h4s -->
    <h1 data-expected-offset="4"><!-- Level 4, h1 + 2 + 1 = 4 --></h1>
    <h2 data-expected-offset="5"><!-- Level 5, h2 + 2 + 1 = 5 --></h2>
    <div headingreset title="container headingreset">
      <!-- h1s are now h1s -->
      <h1 data-expected-offset="1"><!-- Level 1, h1 (headingreset)--></h1>
    </div>
    <dialog open title="container dialog">
      <!-- non-modal dialogs do not headingreset, h1s are still h4s -->
      <h1 data-expected-offset="4"><!-- Level 4, h1 + 2 + 1 = 4 --></h1>
      <h1 data-expected-offset="1" headingreset>
        <!-- Level 1, h1 (headingreset) -->
      </h1>
    </dialog>
  </div>
</div>
<!-- Clamping -->
<div headingoffset="8" title="container headingoffset=8">
  <!-- h1s are now h9s -->
  <h1 data-expected-offset="9"><!-- Level 9, h1 + 8 --></h1>
  <h2 data-expected-offset="9"><!-- Level 9, h2 + 8 (clamped) --></h2>
  <h3 data-expected-offset="9"><!-- Level 9, h3 + 8 (clamped) --></h3>
  <h4 data-expected-offset="9"><!-- Level 9, h4 + 8 (clamped) --></h4>
  <h5 data-expected-offset="9"><!-- Level 9, h5 + 8 (clamped) --></h5>
  <h6 data-expected-offset="9"><!-- Level 9, h6 + 8 (clamped) --></h6>
  <div headingreset title="container headingreset">
    <!-- h1s are now h1s -->
    <h1 data-expected-offset="1"><!-- Level 1, h1 (headingreset)--></h1>
  </div>
  <dialog open title="container dialog">
    <!-- non-modal dialogs do not headingreset, h1s are still h4s -->
    <h1 data-expected-offset="9"><!-- Level 9, h1 + 8 --></h1>
  </dialog>
</div>
<!-- Negative headingoffsets are clamped to `0` -->
<div headingoffset="-3" title="container headingoffset=-3">
  <h1 data-expected-offset="1"><!-- Level 1, h1 + (-3 clamped to 0) --></h1>
  <h2 data-expected-offset="2"><!-- Level 2, h2 + (-3 clamped to 0) --></h2>
  <h3 data-expected-offset="3"><!-- Level 3, h3 + (-3 clamped to 0) --></h3>
  <h4 data-expected-offset="4"><!-- Level 4, h4 + (-3 clamped to 0) --></h4>
  <h5 data-expected-offset="5"><!-- Level 5, h5 + (-3 clamped to 0) --></h5>
  <h6 data-expected-offset="6"><!-- Level 6, h6 + (-3 clamped to 0) --></h6>
  <div headingreset title="container headingreset">
    <!-- h1s are now h1s -->
    <h1 data-expected-offset="1"><!-- Level 1, h1 (headingreset)--></h1>
  </div>
  <dialog open title="container dialog">
    <!-- non-modal dialogs do not headingreset, h1s are still h1s -->
    <h1 data-expected-offset="1"><!-- Level 1, h1 + (-3 clamped to 0) --></h1>
  </dialog>
</div>
<!-- Resetting applies after headingOffset -->
<div headingreset title="container headingreset + headingoffset">
  <div headingoffset="2" headingreset>
    <div headingoffset="2">
      <h1 data-expected-offset="5"><!-- Level 5, h1 + 2 + 2 = 5 --></h1>
      <h2 data-expected-offset="6"><!-- Level 6, h2 + 2 + 2 = 6 --></h2>
      <h3 data-expected-offset="7"><!-- Level 7, h3 + 2 + 2 = 7 --></h3>
      <h4 data-expected-offset="8"><!-- Level 8, h4 + 2 + 2 = 8 --></h4>
      <h5 data-expected-offset="9"><!-- Level 9, h5 + 2 + 2 = 9 --></h5>
      <h6 data-expected-offset="9">
        <!-- Level 9, h6 + 2 + 2 = (10 clamped to 9) -->
      </h6>
      <h1 data-expected-offset="3" headingoffset="2" headingreset>
        <!-- Level 3, h1 + 2 + (headingreset) -->
      </h1>
      <h2 data-expected-offset="4" headingoffset="2" headingreset>
        <!-- Level 4, h2 + 2 + (headingreset) -->
      </h2>
    </div>
  </div>
</div>
<!-- Ensure shadow roots work -->
<div headingoffset="1" title="container shadowroot headingoffset=1">
  <template shadowrootmode="open">
    <h1 data-expected-offset="2"><!-- Level 2, h1 + 1 --></h1>
    <h2 data-expected-offset="2" headingreset>
      <!-- Level 2, h2 (headingreset) -->
    </h2>
  </template>
</div>
<!-- Ensure slotted elements are correctly set -->
<div headingoffset="1" title="container shadowroot slotted headingoffset=1">
  <template shadowrootmode="open">
    <h1 data-expected-offset="2"><!-- Level 2, h1 + 1 --></h1>
    <h2 data-expected-offset="3"><!-- Level 3, h2 + 1 --></h2>
    <h3 data-expected-offset="3" headingreset>
      <!-- Level 3, h3 (headingreset) -->
    </h3>
    <slot></slot>
  </template>
  <h1><!-- Level 2, h1 + 1 --></h1>
</div>
<!-- Ensure slotted elements respect their parents -->
<div
  headingoffset="1"
  title="container shadowroot slotted with container headingoffset=1"
>
  <template shadowrootmode="open">
    <h1 data-expected-offset="2"><!-- Level 2, h1 + 1 --></h1>
    <div headingoffset="1" title="container inside shadowroot headingoffset=1">
      <slot></slot>
    </div>
  </template>
  <h2 data-expected-offset="4"><!-- Level 4, h2 + 1 + 1 --></h2>
  <h4 data-expected-offset="6"><!-- Level 6, h4 + 1 + 1 --></h4>
  <h4 data-expected-offset="4" headingreset>
    <!-- Level 4, h4 (headingreset) -->
  </h4>
</div>
<!-- Ensure the slot can be decorated with headingoffset -->
<div
  headingoffset="1"
  title="container shadowroot slot with attr headingoffset=1"
>
  <template shadowrootmode="open">
    <h1 data-expected-offset="2"><!-- Level 2, h1 + 1 --></h1>
    <slot headingoffset="1"></slot>
  </template>
  <h2 data-expected-offset="4"><!-- Level 4, h2 + 1 + 1 --></h2>
</div>
<h1 data-expected-offset="2" headingoffset="1"><!-- Level 2, h1 + 1 --></h1>
<h2 data-expected-offset="3" headingoffset="1"><!-- Level 3, h2 + 1 --></h2>
<h1 data-expected-offset="3" headingoffset="2"><!-- Level 3, h1 + 2--></h1>
<h2 data-expected-offset="4" headingoffset="2"><!-- Level 4, h2 + 2 --></h2>
<h1 data-expected-offset="2" headingoffset="1" headingreset>
  <!-- Level 2, h1 + 1 (headingreset) -->
</h1>
<h2 data-expected-offset="3" headingoffset="1" headingreset>
  <!-- Level 3, h2 + 1 (headingreset) -->
</h2>
<h1 data-expected-offset="3" headingoffset="2" headingreset>
  <!-- Level 3, h1 + 2 (headingreset) -->
</h1>
<h2 data-expected-offset="4" headingoffset="2" headingreset>
  <!-- Level 4, h2 + 2 (headingreset) -->
</h2>
<h1 data-expected-offset="9" headingoffset="20" headingreset>
  <!-- Level 9, h1 + 20 (clamped)  -->
</h1>
<h2 data-expected-offset="9" headingoffset="20" headingreset>
  <!-- Level 9, h2 + 20 (clamped) -->
</h2>
<h1 data-expected-offset="1" headingoffset="0" headingreset>
  <!-- Level 1, h1 + 0 -->
</h1>
<h2 data-expected-offset="2" headingoffset="0" headingreset>
  <!-- Level 2, h2 + 0 -->
</h2>
<div title="container with no attr">
  <h1 data-expected-offset="2" headingoffset="1"><!-- Level 2, h1 + 1 --></h1>
  <h2 data-expected-offset="3" headingoffset="1"><!-- Level 3, h2 + 1 --></h2>
  <h1 data-expected-offset="2" headingoffset="1" headingreset>
    <!-- Level 2, h1 + 1 -->
  </h1>
  <h2 data-expected-offset="3" headingoffset="1" headingreset>
    <!-- Level 3, h2 + 1 -->
  </h2>
  <h1 data-expected-offset="1" headingoffset="-1" headingreset>
    <!-- Level 1, h1 + (-1 clamped to 0) -->
  </h1>
  <h2 data-expected-offset="2" headingoffset="-1" headingreset>
    <!-- Level 2, h2 + (-1 clamped to 0) -->
  </h2>
</div>
<div headingreset title="many nested + and - values">
  <div headingoffset="-1">
    <div headingoffset="3">
      <div headingoffset="-6">
        <div headingoffset="1">
          <h1 data-expected-offset="5">
            <!-- Level 5, h1 + 1 + (-6 clamped to 0) + 3 + (-1 clamped to 0) -->
          </h1>
        </div>
      </div>
    </div>
  </div>
</div>
<h1 data-expected-offset="9" headingoffset="9" aria-level="3">
  <!-- Level 3 -->
</h1>

<div headingoffset="9" id="modalParent">
  <dialog id="modal">
    <h1></h1>
  </dialog>
</div>

<script>
  const attribute = (el, val) => el.setAttribute("headingoffset", val);
  const property = (el, val) => (el.headingOffset = val);
  const levels = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  const matchAllLevels = (el) =>
    levels.map((l) => el.matches(`:heading(${l})`));

  for (const fn of [attribute, property]) {
    test(function () {
      const el = document.createElement("h1");
      assert_true(el.matches(":heading"), `h1 should match :heading`);
      assert_true(el.matches(":heading(1)"), `h1 should match :heading(1)`);
      assert_equals(
        el.headingOffset,
        0,
        `h1 has an initial headingOffset of 0`,
      );
      fn(el, 3);
      assert_equals(el.headingOffset, 3, `h1 now has a headingOffset of 3`);
      assert_false(
        el.matches(":heading(1)"),
        `h1[headingoffset=3] should no longer match :heading(1)`,
      );
      assert_true(
        el.matches(":heading(4)"),
        `h1[headingoffset=3] should match :heading(4)`,
      );
    }, `headingoffset (set via ${fn.name}) should change the level a heading matches against`);

    test(function () {
      const parent = document.createElement("div");
      const el = document.createElement("h1");
      parent.append(el);
      assert_true(el.matches(":heading"), `h1 should match :heading`);
      assert_true(el.matches(":heading(1)"), `h1 should match :heading(1)`);
      assert_equals(
        parent.headingOffset,
        0,
        `parent has an initial headingOffset of 0`,
      );
      fn(parent, 3);
      assert_equals(parent.headingOffset, 3, `h1 now has a headingOffset of 3`);
      assert_false(
        el.matches(":heading(1)"),
        `h1[headingoffset=3] should no longer match :heading(1)`,
      );
      assert_true(
        el.matches(":heading(4)"),
        `div[headingoffset=3] h1 should match :heading(4)`,
      );
      const bools = matchAllLevels(el);
      const expected_bools = levels.map((l) => l == 4);
      assert_array_equals(
        bools,
        expected_bools,
        `${el.outerHTML} should match only the expected heading level`,
      );
      assert_false(el.headingReset, "h1 has an initial headingReset=false");
      el.headingReset = true;
      assert_true(el.headingReset, "h1 now has a headingReset of true");
      assert_false(
        el.matches(":heading(4)"),
        `div[headingoffset=3] h1[headingreset] should not match :heading(4)`,
      );
      assert_true(
        el.matches(":heading(1)"),
        `div[headingoffset=3] h1[headingreset] should match :heading(1)`,
      );
      const reset_bools = matchAllLevels(el);
      const reset_expected_bools = levels.map((l) => l == 1);
      assert_array_equals(
        reset_bools,
        reset_expected_bools,
        `${el.outerHTML} should match only the expected heading level`,
      );
    }, `headingoffset (set via ${fn.name}) should change the level when a parent changes offset`);
  }

  test(function (t) {
    const el = modal.querySelector("h1");
    assert_false(modal.headingReset, `modal dialog has not set heading reset`);
    modal.showModal();
    t.add_cleanup(() => modal.close());

    assert_true(modal.headingReset, `modal dialogs return headingReset true`);
    assert_true(el.matches(":heading"), `h1 inside modal should match :heading`);
    assert_true(el.matches(":heading(1)"), `h1 inside modal should match :heading(1)`);
    const bools = matchAllLevels(el);
    const expected_bools = levels.map((l) => l == 1);
    assert_array_equals(
      bools,
      expected_bools,
      `${el.outerHTML} should match only the expected heading level`,
    );
  }, `headingoffset should not impact modals or explicit headingreset containers`);

  let i = 0;
  for (const el of document.querySelectorAll("[data-expected-offset]")) {
    i += 1;
    test(function () {
      const expected = parseInt(el.getAttribute("data-expected-offset"));
      assert_true(
        expected > 0 && expected < 10,
        "expected offset should be a level from 1 to 9",
      );
      assert_true(
        el.matches(":heading"),
        `${el.outerHTML} should match :heading`,
      );
      const bools = matchAllLevels(el);
      const expected_bools = levels.map((l) => l == expected);
      assert_true(
        el.matches(`:heading(${expected})`),
        `${el.outerHTML} should match :heading(${expected})`,
      );
      assert_array_equals(
        bools,
        expected_bools,
        `${el.outerHTML} should match only the expected heading level`,
      );
    }, `case ${i}: heading level for ${el.outerHTML} should match based on expected document structure`);
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 507,
        "byte_start": 502,
        "col": 60,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 572,
        "byte_start": 567,
        "col": 60,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 637,
        "byte_start": 632,
        "col": 60,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 797,
        "byte_start": 792,
        "col": 66,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 868,
        "byte_start": 863,
        "col": 66,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1028,
        "byte_start": 1023,
        "col": 70,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 4) follows the heading “h1” (with computed level 1), skipping 2 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 1190,
        "byte_start": 1161,
        "col": 7,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1227,
        "byte_start": 1222,
        "col": 68,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1332,
        "byte_start": 1327,
        "col": 7,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 9) follows the heading “h1” (with computed level 1), skipping 7 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 1497,
        "byte_start": 1468,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1526,
        "byte_start": 1521,
        "col": 56,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1597,
        "byte_start": 1592,
        "col": 66,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1668,
        "byte_start": 1663,
        "col": 66,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1739,
        "byte_start": 1734,
        "col": 66,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1810,
        "byte_start": 1805,
        "col": 66,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1881,
        "byte_start": 1876,
        "col": 66,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2035,
        "byte_start": 2030,
        "col": 68,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 9) follows the heading “h1” (with computed level 1), skipping 7 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 2189,
        "byte_start": 2160,
        "col": 5,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2218,
        "byte_start": 2213,
        "col": 58,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2349,
        "byte_start": 2290,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2426,
        "byte_start": 2421,
        "col": 72,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2503,
        "byte_start": 2498,
        "col": 72,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2580,
        "byte_start": 2575,
        "col": 72,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2657,
        "byte_start": 2652,
        "col": 72,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2734,
        "byte_start": 2729,
        "col": 72,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2811,
        "byte_start": 2806,
        "col": 72,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2965,
        "byte_start": 2960,
        "col": 68,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3164,
        "byte_start": 3159,
        "col": 74,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 5) follows the heading “h1” (with computed level 1), skipping 3 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 3399,
        "byte_start": 3370,
        "col": 7,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3436,
        "byte_start": 3431,
        "col": 68,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3509,
        "byte_start": 3504,
        "col": 68,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3582,
        "byte_start": 3577,
        "col": 68,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3655,
        "byte_start": 3650,
        "col": 68,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3728,
        "byte_start": 3723,
        "col": 68,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3833,
        "byte_start": 3828,
        "col": 7,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3962,
        "byte_start": 3957,
        "col": 7,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4091,
        "byte_start": 4086,
        "col": 7,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4319,
        "byte_start": 4314,
        "col": 58,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4418,
        "byte_start": 4413,
        "col": 5,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4665,
        "byte_start": 4660,
        "col": 58,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4728,
        "byte_start": 4723,
        "col": 58,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4827,
        "byte_start": 4822,
        "col": 5,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 4895,
        "byte_start": 4890,
        "col": 31,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5152,
        "byte_start": 5147,
        "col": 58,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5342,
        "byte_start": 5337,
        "col": 60,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h4” (with computed level 5) follows the heading “h2” (with computed level 3), skipping 1 heading level.",
      "severity": "Warning",
      "span": {
        "byte_end": 5374,
        "byte_start": 5345,
        "col": 3,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5407,
        "byte_start": 5402,
        "col": 60,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5500,
        "byte_start": 5495,
        "col": 3,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5755,
        "byte_start": 5750,
        "col": 58,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5870,
        "byte_start": 5865,
        "col": 60,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 5954,
        "byte_start": 5949,
        "col": 72,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6031,
        "byte_start": 6026,
        "col": 72,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6107,
        "byte_start": 6102,
        "col": 71,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6184,
        "byte_start": 6179,
        "col": 72,
        "line": 139
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6293,
        "byte_start": 6288,
        "col": 1,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6402,
        "byte_start": 6397,
        "col": 1,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6511,
        "byte_start": 6506,
        "col": 1,
        "line": 148
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6620,
        "byte_start": 6615,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 9) follows the heading “h2” (with computed level 4), skipping 4 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 6682,
        "byte_start": 6621,
        "col": 1,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6682,
        "byte_start": 6621,
        "col": 1,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6727,
        "byte_start": 6722,
        "col": 1,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6789,
        "byte_start": 6728,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6833,
        "byte_start": 6828,
        "col": 1,
        "line": 157
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 6927,
        "byte_start": 6922,
        "col": 1,
        "line": 160
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7021,
        "byte_start": 7016,
        "col": 1,
        "line": 163
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7137,
        "byte_start": 7132,
        "col": 74,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7216,
        "byte_start": 7211,
        "col": 74,
        "line": 166
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7316,
        "byte_start": 7311,
        "col": 3,
        "line": 169
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7416,
        "byte_start": 7411,
        "col": 3,
        "line": 172
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7480,
        "byte_start": 7419,
        "col": 3,
        "line": 173
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7533,
        "byte_start": 7528,
        "col": 3,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7597,
        "byte_start": 7536,
        "col": 3,
        "line": 176
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7650,
        "byte_start": 7645,
        "col": 3,
        "line": 178
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7738,
        "byte_start": 7714,
        "col": 3,
        "line": 181
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7797,
        "byte_start": 7773,
        "col": 7,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 5) follows the heading “h2” (with computed level 2), skipping 2 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 7869,
        "byte_start": 7840,
        "col": 11,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 7966,
        "byte_start": 7961,
        "col": 11,
        "line": 187
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h1” (with computed level 9) follows the heading “h1” (with computed level 5), skipping 3 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 8084,
        "byte_start": 8022,
        "col": 1,
        "line": 193
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8084,
        "byte_start": 8022,
        "col": 1,
        "line": 193
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 8109,
        "byte_start": 8104,
        "col": 1,
        "line": 195
      }
    },
    {
      "category": "Html",
      "code": "html.headingoffset.range",
      "message": "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8151,
        "byte_start": 8111,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 8187,
        "byte_start": 8182,
        "col": 9,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/sections/headingoffset-and-headingreset.html"
}
```
