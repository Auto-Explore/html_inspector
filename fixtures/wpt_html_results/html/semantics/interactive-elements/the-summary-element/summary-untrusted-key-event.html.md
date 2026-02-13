# html/semantics/interactive-elements/the-summary-element/summary-untrusted-key-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-summary-element/summary-untrusted-key-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
<title>Summary</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<details>
  <summary>Summary</summary>
  Details
</details>
<script type="module">
const details = document.querySelector("details");
details.addEventListener("toggle",
  (e) => assert_true(false, 'details should not be toggled'));

const summary = document.querySelector("summary");
summary.addEventListener("click",
  (e) => assert_true(false, `summary should not be clicked`));

test(() => {
  // keyCode: Enter
  summary.dispatchEvent(
    new KeyboardEvent("keypress", {
      keyCode: 13,
    })
  );

  // key: Enter
  summary.dispatchEvent(
    new KeyboardEvent("keypress", {
      key: "Enter",
    })
  );

  // keyCode: Space
  summary.dispatchEvent(
    new KeyboardEvent("keypress", {
      keyCode: 32,
    })
  );

  // key: Space
  summary.dispatchEvent(
    new KeyboardEvent("keypress", {
      key: " ",
    })
  );
}, `Dispatching untrusted keypress events to summary should not cause click event`);

test(() => {
  // keyCode: Enter
  summary.dispatchEvent(
    new KeyboardEvent("keydown", {
      keyCode: 13,
    })
  );
  summary.dispatchEvent(
    new KeyboardEvent("keyup", {
      keyCode: 13,
    })
  );

  // key: Enter
  summary.dispatchEvent(
    new KeyboardEvent("keydown", {
      key: "Enter",
    })
  );
  summary.dispatchEvent(
    new KeyboardEvent("keyup", {
      key: "Enter",
    })
  );

  // keyCode: Space
  summary.dispatchEvent(
    new KeyboardEvent("keydown", {
      keyCode: 32,
    })
  );
  summary.dispatchEvent(
    new KeyboardEvent("keyup", {
      keyCode: 32,
    })
  );

  // key: Space
  summary.dispatchEvent(
    new KeyboardEvent("keydown", {
      key: " ",
    })
  );
  summary.dispatchEvent(
    new KeyboardEvent("keyup", {
      key: " ",
    })
  );
}, `Dispatching untrusted keyup/keydown events to summary should not cause click event`);
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-summary-element/summary-untrusted-key-event.html"
}
```
