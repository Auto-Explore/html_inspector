# html/semantics/forms/the-button-element/button-untrusted-key-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-untrusted-key-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
<title>Forms</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<form id="input_form">
  <button name="submitButton" type="submit">Submit</button>
  <button name="resetButton" type="reset">Reset</button>
  <button name="buttonButton" type="button">Button</button>
</form>
<script type="module">
const form = document.querySelector("form");
form.addEventListener("submit", (e) => {
  e.preventDefault();
  assert_true(false, 'form should not be submitted');
});

for (const button of document.querySelectorAll("button")) {
  button.addEventListener("click", function(e) {
    assert_true(false, `${button.type} button should not be clicked`);
  });
}

// Create and append button elements
for (const button of document.querySelectorAll("button")) {
  test(() => {
    // keyCode: Enter
    button.dispatchEvent(
      new KeyboardEvent("keypress", {
        keyCode: 13,
      })
    );

    // key: Enter
    button.dispatchEvent(
      new KeyboardEvent("keypress", {
        key: "Enter",
      })
    );

    // keyCode: Space
    button.dispatchEvent(
      new KeyboardEvent("keypress", {
        keyCode: 32,
      })
    );

    // key: Space
    button.dispatchEvent(
      new KeyboardEvent("keypress", {
        key: " ",
      })
    );
  }, `Dispatching untrusted keypress events to ${button.type} button should not cause click event`);

  test(() => {
    // keyCode: Enter
    button.dispatchEvent(
      new KeyboardEvent("keydown", {
        keyCode: 13,
      })
    );
    button.dispatchEvent(
      new KeyboardEvent("keyup", {
        keyCode: 13,
      })
    );

    // key: Enter
    button.dispatchEvent(
      new KeyboardEvent("keydown", {
        key: "Enter",
      })
    );
    button.dispatchEvent(
      new KeyboardEvent("keyup", {
        key: "Enter",
      })
    );

    // keyCode: Space
    button.dispatchEvent(
      new KeyboardEvent("keydown", {
        keyCode: 32,
      })
    );
    button.dispatchEvent(
      new KeyboardEvent("keyup", {
        keyCode: 32,
      })
    );

    // key: Space
    button.dispatchEvent(
      new KeyboardEvent("keydown", {
        key: " ",
      })
    );
    button.dispatchEvent(
      new KeyboardEvent("keyup", {
        key: " ",
      })
    );
  }, `Dispatching untrusted keyup/keydown events to ${button.type} button should not cause click event`);
}
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
  "source_name": "html/semantics/forms/the-button-element/button-untrusted-key-event.html"
}
```
