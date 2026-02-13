# html/semantics/scripting-1/the-script-element/script-type-whitespace.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-type-whitespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>&lt;script type> non-ASCII whitespace handling</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
function testParserInsertedDidNotRun(description) {
  test(() => assert_false(window.ran),
       "Script shouldn't run with " + description + " (parser-inserted)");
  window.ran = false;
}
</script>

<script>window.ran = false;</script>
<script type="text/javascript&#x000B;">window.ran = true;</script>
<script>testParserInsertedDidNotRun("type=\"text/javascript&#x000B;\"");</script>

<script type="text/javascript&#x0085;">window.ran = true;</script>
<script>testParserInsertedDidNotRun("type=\"text/javascript&#x0085;\"");</script>

<script type="text/javascript&#x00A0;">window.ran = true;</script>
<script>testParserInsertedDidNotRun("type=\"text/javascript&#x00A0;\"");</script>

<script type="text/javascript&#x1680;">window.ran = true;</script>
<script>testParserInsertedDidNotRun("type=\"text/javascript&#x1680;\"");</script>

<script type="text/javascript&#x3000;">window.ran = true;</script>
<script>testParserInsertedDidNotRun("type=\"text/javascript&#x3000;\"");</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 466,
        "byte_start": 427,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 443,
        "byte_start": 442,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_c1_controls",
      "message": "A numeric character reference expanded to the C1 controls range.",
      "severity": "Warning",
      "span": {
        "byte_end": 593,
        "byte_start": 592,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 766,
        "byte_start": 727,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 916,
        "byte_start": 877,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1066,
        "byte_start": 1027,
        "col": 1,
        "line": 26
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-type-whitespace.html"
}
```
