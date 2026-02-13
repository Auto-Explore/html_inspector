# html/semantics/forms/the-label-element/control-active-with-multiple-clicks.html

Counts:
- errors: 6
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/control-active-with-multiple-clicks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics-other.html#being-actively-pointed-at">
<meta name="assert" content="Additionally, any element that is the labeled control of a label element that is currently matching :active, also matches :active.">
<meta name="timeout" content="long">
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<style>
* {
  font-size: 20px;
}
button:active, input:active, meter:active, progress:active, select:active, textarea:active {
  outline: 1px solid red;
}
*:focus {
  outline: none;
}
</style>
</head>
<body>

<!-- Testing instructions-->
<h1>Manual Testing Instructions</h1>
<p>
  The test is automatic, but if you wish to test it manually, you can do so by following the steps below.
</p>
<ol>
    <li>
      Click on the label of the interactive element of your choosing. For example,
      you can click on the label of the first button element by clicking on the text "button1".
      Do not click on anything afterwards before the next step.
    </li>
    <li>
      Expected behavior: While mousedowning, the element on the right of the title text
      should have received a red outline for the duration of the mousedown.
    </li>
    <li>
      Click again on the same label (do not double-click, wait a second before clicking again).
    </li>
    <li>
      Expected behavior: Again, while mousedowning on the click,
      the element on the right of the title text should have received a red outline.
    </li>
    <li>
      Expected behavior: The above steps should pass for all the labels on this page.
    </li>
</ol>

<!-- Button -->
<div>
  <label id="button1-label" for="button1">button1</label>
  <button id="button1" class="control"></button>
  <label id="button2-label">button2<button id="button2" class="control"></button></label>
</div>

<!-- Default input -->
<div>
  <label id="input1-label" for="input1">input1</label>
  <input id="input1" class="control"></input>
  <label id="input2-label">input2<input id="input2" class="control"></input></label>
</div>

<!-- Checkbox input -->
<div>
  <label id="checkbox1-label" for="checkbox1">checkbox1</label>
  <input type="checkbox" id="checkbox1" class="control"></input>
  <label id="checkbox2-label">checkbox2
    <input type="checkbox" id="checkbox2" class="control"></input>
  </label>
</div>

<!-- Meter -->
<div>
  <label id="meter1-label" for="meter1">meter1</label>
  <meter id="meter1" class="control"></meter>
  <label id="meter2-label">meter2<meter id="meter2" class="control"></meter></label>
</div>

<!-- Progress -->
<div>
  <label id="progress1-label" for="progress1">progress1</label>
  <progress id="progress1" class="control"></progress>
  <label id="progress2-label">progress2<progress id="progress2" class="control"></progress></label>
</div>

<!-- Select -->
<div>
  <label id="select1-label" for="select1">select1</label>
  <select id="select1" class="control"></select>
  <label id="select2-label">select2<select id="select2" class="control"></select></label>
</div>

<!-- Textarea -->
<div>
  <label id="textarea1-label" for="textarea1">textarea1</label>
  <textarea id="textarea1" class="control"></textarea>
  <label id="textarea2-label">textarea2<textarea id="textarea2" class="control"></textarea></label>
</div>

<script>

function assertIsActive(shouldBeActive, element, name, whenDescription) {
  assertMessage = (
    name + " element " + element.id +" should" + (shouldBeActive ? " " : " not ") +
    "be :active when " + whenDescription + "."
  );
  const isActive = document.querySelector("#" + element.id + ":active") == element;
  assert_equals(isActive, shouldBeActive, assertMessage);
}

async function testWith(control, label, isFirstClick) {
  clickNumSuffix = isFirstClick ? " on the first click" : " on the second click";

  await ((new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerDown())
    .send());

  assertIsActive(true, control, "Control", "label is mousedowned" + clickNumSuffix);
  assertIsActive(true, label, "Label", "label is mousedowned" + clickNumSuffix);

  await ((new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerUp())
    .send());

  assertIsActive(false, control, "Control", "label is no longer mousedowned" + clickNumSuffix);
  assertIsActive(false, label, "Label", "label is no longer mousedowned" + clickNumSuffix);
}

// If testharness.js or test_driver.js is not available then the test is
// degraded into a manual test.
if (typeof assert_equals !== "undefined" &&
  typeof test_driver !== "undefined" &&
  typeof test_driver.Actions !== "undefined") {
  window.onload = () => {
    promise_test(async t => {
      const labels = Array.from(document.querySelectorAll("label"));
      const controlLabelPairs = labels.map((label) => [label.control, label]);

      for (const pair of controlLabelPairs) {
        await testWith(...pair, true);
        await testWith(...pair, false);
      }
    }, "Control should be :active on mousedown when clicking the label twice in succession.");
  }
}

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 2222,
        "byte_start": 2214,
        "col": 38,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 2299,
        "byte_start": 2291,
        "col": 69,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 2474,
        "byte_start": 2466,
        "col": 57,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 2581,
        "byte_start": 2573,
        "col": 59,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.meter.missing_value",
      "message": "Element “meter” is missing required attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2714,
        "byte_start": 2679,
        "col": 3,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.meter.missing_value",
      "message": "Element “meter” is missing required attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2791,
        "byte_start": 2756,
        "col": 34,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.label.for.must_reference_non_hidden_control",
      "message": "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
      "severity": "Error",
      "span": {
        "byte_end": 2662,
        "byte_start": 2624,
        "col": 3,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.label.for.must_reference_non_hidden_control",
      "message": "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
      "severity": "Error",
      "span": {
        "byte_end": 2886,
        "byte_start": 2842,
        "col": 3,
        "line": 86
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
  "source_name": "html/semantics/forms/the-label-element/control-active-with-multiple-clicks.html"
}
```
