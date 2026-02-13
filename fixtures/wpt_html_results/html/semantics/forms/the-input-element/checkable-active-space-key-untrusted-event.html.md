# html/semantics/forms/the-input-element/checkable-active-space-key-untrusted-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/checkable-active-space-key-untrusted-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Tests active state of checkbox/radio when pressing space key emulated with untrusted key events</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<input type="checkbox">
<input type="radio">
<script>
function sendSpaceKeyEvent(eventType, target) {
  const eventData = { keyCode: 32, which: 32, key: " ", code: "Space"};
  const spaceKeyEvent = new KeyboardEvent(eventType, eventData);
  target.dispatchEvent(spaceKeyEvent);
}

test(t => {
  const checkbox = document.querySelector("input[type=checkbox]");
  checkbox.focus();
  sendSpaceKeyEvent("keydown", checkbox);
  t.add_cleanup(() => {
    sendSpaceKeyEvent("keyup", checkbox);
  });
  assert_equals(
    document.querySelector("input:active"),
    null,
    "The checkbox shouldn't be activated"
  );
}, "Space key shouldn't active the checkbox when space key press is emulated by untrusted events");

test(t => {
  const radio = document.querySelector("input[type=radio]");
  radio.focus();
  sendSpaceKeyEvent("keydown", radio);
  t.add_cleanup(() => {
    sendSpaceKeyEvent("keyup", radio);
  });
  assert_equals(
    document.querySelector("input:active"),
    null,
    "The radio shouldn't be activated"
  );
}, "Space key shouldn't active the radio when space key press is emulated by untrusted events");
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
  "source_name": "html/semantics/forms/the-input-element/checkable-active-space-key-untrusted-event.html"
}
```
