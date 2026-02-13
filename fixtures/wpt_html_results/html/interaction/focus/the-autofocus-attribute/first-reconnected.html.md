# html/interaction/focus/the-autofocus-attribute/first-reconnected.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/first-reconnected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/utils.js"></script>

<input autofocus id="i1">
<input autofocus id="i2">
<script>
"use strict";

promise_test(async () => {
  const input1 = document.querySelector("#i1");
  const input2 = document.querySelector("#i2");
  input1.remove();
  input2.parentNode.insertBefore(input1, input2);

  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, input2);
}, 'The second autofocus element wins if the first autofocus element was ' +
   'disconnected and reconnected before flushing the autofocus candidates.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 240,
        "byte_start": 215,
        "col": 1,
        "line": 8
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/first-reconnected.html"
}
```
