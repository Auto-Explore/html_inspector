# html/semantics/scripting-1/the-script-element/script-type-and-language-empty.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-type-and-language-empty.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script @type and @language: empty strings</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- Setup -->
<script>
window.run1 = window.run2 = window.run3 = window.run4 = false;
</script>

<!-- Systems under test -->
<script type="">
window.run1 = true;
</script>

<script type="" language="foo">
window.run2 = true;
</script>

<script type="" language="">
window.run3 = true;
</script>

<script language="">
window.run4 = true;
</script>

<!-- Asserts -->
<script>
test(() => {
  assert_true(window.run1);
}, "A script with empty type and no language should run");

test(() => {
  assert_true(window.run2);
}, "A script with empty type and a random language should run");

test(() => {
  assert_true(window.run3);
}, "A script with empty type and empty language should run");

test(() => {
  assert_true(window.run4);
}, "A script with no type and empty language should run");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.language.obsolete",
      "message": "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 605,
        "byte_start": 574,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.script.language.obsolete",
      "message": "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 665,
        "byte_start": 637,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.script.language.obsolete",
      "message": "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 717,
        "byte_start": 697,
        "col": 1,
        "line": 27
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-type-and-language-empty.html"
}
```
