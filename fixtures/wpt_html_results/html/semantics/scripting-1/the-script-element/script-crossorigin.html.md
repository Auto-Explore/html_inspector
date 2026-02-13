# html/semantics/scripting-1/the-script-element/script-crossorigin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-crossorigin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTMLScriptElement: crossOrigin IDL attribute</title>
<link rel="author" title="KiChjang" href="mailto:kungfukeith11@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#cors-settings-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script id="script1"></script>
<script id="script2" crossorigin=""></script>
<script id="script3" crossorigin="foo"></script>
<script id="script4" crossorigin="anonymous"></script>
<script id="script5" crossorigin="use-credentials"></script>
<script>
test(function() {
  var script1 = document.getElementById("script1");
  var script2 = document.getElementById("script2");
  var script3 = document.getElementById("script3");
  var script4 = document.getElementById("script4");
  var script5 = document.getElementById("script5");

  assert_equals(script1.crossOrigin, null, "Missing value default should be null");
  assert_equals(script2.crossOrigin, "anonymous", "Empty string should map to anonymous");
  assert_equals(script3.crossOrigin, "anonymous", "Invalid value default should be anonymous");
  assert_equals(script4.crossOrigin, "anonymous", "anonymous should be parsed correctly");
  assert_equals(script5.crossOrigin, "use-credentials", "use-credentials should be parsed correctly");

  script1.crossOrigin = "bar";
  assert_equals(script1.crossOrigin, "anonymous", "Setting to invalid value would default to anonymous");

  script2.crossOrigin = null;
  assert_equals(script2.crossOrigin, null, "Resetting to null should work");

  script4.crossOrigin = "use-credentials";
  assert_equals(script4.crossOrigin, "use-credentials", "Switching from anonymous to use-credentials should work");

  script5.crossOrigin = "anonymous";
  assert_equals(script5.crossOrigin, "anonymous", "Switching from use-credentials to anonymous should work");
}, document.title);
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-crossorigin.html"
}
```
