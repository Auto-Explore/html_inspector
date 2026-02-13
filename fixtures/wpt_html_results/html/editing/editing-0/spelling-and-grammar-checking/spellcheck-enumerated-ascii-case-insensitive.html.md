# html/editing/editing-0/spelling-and-grammar-checking/spellcheck-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/spelling-and-grammar-checking/spellcheck-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#attr-spellcheck">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="@spellcheck values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<!--
  While <div> and <span> aren’t defined as “checkable for the purposes of this
  feature”, this has no effect on the attribute’s state.

  We wrap the <span> elements under test with <div> elements so the checking
  enabled algorithm stops at step 4 (ancestor content attribute), before steps
  relying on user-agent-defined behavior (see [#concept-spellcheck-default]).
-->
<div spellcheck="true"><span spellcheck="false"></span></div>
<div spellcheck="true"><span spellcheck="FaLsE"></span></div>
<div spellcheck="true"><span spellcheck="falſe"></span></div>
<script>
const span = document.querySelectorAll("span");

test(() => {
  assert_equals(span[0].spellcheck, false, "lowercase valid");
  assert_equals(span[1].spellcheck, false, "mixed case valid");
  assert_equals(span[2].spellcheck, true, "non-ASCII invalid");
}, "keyword false");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.spellcheck.invalid",
      "message": "Bad value “falſe” for attribute “spellcheck” on element “span”.",
      "severity": "Warning",
      "span": {
        "byte_end": 930,
        "byte_start": 904,
        "col": 24,
        "line": 18
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
  "source_name": "html/editing/editing-0/spelling-and-grammar-checking/spellcheck-enumerated-ascii-case-insensitive.html"
}
```
