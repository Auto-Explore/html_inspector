# html/interaction/focus/the-autofocus-attribute/queue-non-focusable.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/queue-non-focusable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/utils.js"></script>

<textarea autofocus disabled></textarea>
<select autofocus></select>

<script>
'use strict';

promise_test(async () => {
  const [textarea, select] = document.querySelectorAll('[autofocus]');
  textarea.disabled = false;

  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, textarea);
  assert_not_equals(document.activeElement, select);
}, 'If the first autofocus element is not focusable, but becomes focusable before a frame, it should be focused.');
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
        "byte_end": 225,
        "byte_start": 207,
        "col": 1,
        "line": 7
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/queue-non-focusable.html"
}
```
