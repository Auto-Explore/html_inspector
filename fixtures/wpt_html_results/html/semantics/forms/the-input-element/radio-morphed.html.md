# html/semantics/forms/the-input-element/radio-morphed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/radio-morphed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Morphed radio input</title>
<link rel="author" title="Kagami Sascha Rosylight" href="mailto:krosylight@mozilla.com">
<link rel="help" href="https://html.spec.whatwg.org/#radio-button-state-(type=radio)">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<input id="radio" type="radio" name="name_7" checked>
<input id="text" name="name_7" checked>
<script>
  "use strict";

  test(() => {
    text.type = 'radio';
    assert_false(radio.checked);
  }, "Setting type attribute must unset checkedness of other elements");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.checked.disallowed_for_type",
      "message": "Attribute “checked” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 427,
        "byte_start": 388,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/forms/the-input-element/radio-morphed.html"
}
```
