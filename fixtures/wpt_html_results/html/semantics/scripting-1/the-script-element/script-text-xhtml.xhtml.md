# html/semantics/scripting-1/the-script-element/script-text-xhtml.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-text-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>HTMLScriptElement.text</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-script-text"/>
<script src="/resources/testharness.js"/>
<script src="/resources/testharnessreport.js"/>
</head>
<body>
<div id="log"></div>
<script>
<x>7;</x>
<![CDATA[
  var x = "y";
]]>
</script>
<script>
var script;
setup(function() {
  script = document.body.getElementsByTagName("script")[0];
})
test(function() {
  assert_equals(script.text, '\n\n\n  var x = "y";\n\n')
  assert_equals(script.textContent, '\n7;\n\n  var x = "y";\n\n')
}, "Getter with CDATA section")
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-text-xhtml.xhtml"
}
```
