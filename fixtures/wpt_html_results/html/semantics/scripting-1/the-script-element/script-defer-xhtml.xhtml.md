# html/semantics/scripting-1/the-script-element/script-defer-xhtml.xhtml

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-defer-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
   "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>XHTML Test: HTMLScriptElement - defer</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta charset="utf-8" />
</head>
<body>
<div id="log"></div>

<script>

let script_run_status = "inline";
let t = async_test("the defer script run later");

</script>

<script type="text/javascript" src="defer.js" defer="defer"></script>

<script>

t.step(() => {
  assert_equals(script_run_status, "inline", "the script run status");
  script_run_status = "deferred";
});

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 479,
        "col": 1,
        "line": 20
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-defer-xhtml.xhtml"
}
```
