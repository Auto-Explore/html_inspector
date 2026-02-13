# html/semantics/forms/the-select-element/customizable-select/select-anchor-regression.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-anchor-regression.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:andruud@chromium.org">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/375004874">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
  select,
  ::picker(select) {
    appearance: base-select;
  }

  .amount {
    position: absolute;
    position-anchor: --my-select;
  }

  select:open selectedoption {
    .amount {
      display: none;
    }
  }
</style>

<select>
  <button>
    <selectedoption></selectedoption>
  </button>
  <div>
    <option>
      <div class="title">Potion</div>
      <div class="amount">10</div>
    </option>
  </div>
</select>

<script>
promise_test(async () => {
  await test_driver.click(document.querySelector('select'));
  await new Promise(requestAnimationFrame);
}, 'This test passes if it does not crash.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “selectedoption” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 675,
        "byte_start": 659,
        "col": 5,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “selectedoption” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 675,
        "byte_start": 659,
        "col": 5,
        "line": 30
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-anchor-regression.html"
}
```
