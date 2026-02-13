# html/interaction/focus/the-autofocus-attribute/not-on-first-task.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/not-on-first-task.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The element is not focused during the initial parsing task</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#autofocusing-a-form-control:-the-autofocus-attribute">
<link rel="author" title="Domenic Denicola" href="d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<input autofocus>
<input autofocus>

<script>
"use strict";

test(() => {
  const input = document.querySelector("input");

  assert_equals(document.activeElement, document.body);
  assert_not_equals(document.activeElement, input);
});
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
        "byte_end": 439,
        "byte_start": 422,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/the-autofocus-attribute/not-on-first-task.html"
}
```
