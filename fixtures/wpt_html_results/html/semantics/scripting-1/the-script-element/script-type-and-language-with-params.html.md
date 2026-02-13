# html/semantics/scripting-1/the-script-element/script-type-and-language-with-params.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-type-and-language-with-params.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script @type and @language: unknown type parameters</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#scriptingLanguages">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- Setup -->
<script>
window.run1 = window.run2 = window.run3 = false;
</script>

<!-- Systems under test -->
<script type="text/javascript;charset=UTF-8">
window.run1 = true;
</script>

<script type="text/javascript;x-test=abc">
window.run2 = true;
</script>

<script language="javascript" type="text/javascript;charset=UTF-8">
window.run3 = true;
</script>

<!-- Asserts -->
<script>
test(() => {
  assert_false(window.run1);
}, "A script with a charset param in its type should not run");

test(() => {
  assert_false(window.run2);
}, "A script with an x-test param in its type should not run");

test(() => {
  assert_false(window.run3);
}, "A script with a charset param in its type should not run, even with language=javascript");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.language.javascript.type_mismatch",
      "message": "A “script” element with the “language=\"JavaScript\"” attribute set must not have a “type” attribute whose value is not “text/javascript”.",
      "severity": "Warning",
      "span": {
        "byte_end": 824,
        "byte_start": 757,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.script.language.obsolete",
      "message": "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 824,
        "byte_start": 757,
        "col": 1,
        "line": 24
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-type-and-language-with-params.html"
}
```
