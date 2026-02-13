# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/script-IDL-event-htmlfor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/script-IDL-event-htmlfor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>event and htmlFor IDL attributes of HTMLScriptElement</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-script-event">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-script-htmlfor">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var script = document.createElement("script");
  assert_equals(script.event, "");
  assert_equals(script.htmlFor, "");
})
test(function() {
  var script = document.createElement("script");
  script.setAttribute("event", "blah");
  script.setAttribute("for", "blah");
  assert_equals(script.event, "blah");
  assert_equals(script.htmlFor, "blah");
  assert_equals(script.getAttribute("event"), "blah");
  assert_equals(script.getAttribute("for"), "blah");
})
test(function() {
  var script = document.createElement("script");
  script.setAttribute("event", "blah");
  script.setAttribute("for", "blah");
  script.event = "foo";
  script.htmlFor = "foo";
  assert_equals(script.event, "foo");
  assert_equals(script.htmlFor, "foo");
  assert_equals(script.getAttribute("event"), "foo");
  assert_equals(script.getAttribute("for"), "foo");
})
test(function() {
  var script = document.createElement("script");
  script.setAttribute("event", "blah");
  script.setAttribute("for", "blah");
  script.event = null;
  script.htmlFor = null;
  assert_equals(script.event, "null");
  assert_equals(script.htmlFor, "null");
  assert_equals(script.getAttribute("event"), "null");
  assert_equals(script.getAttribute("for"), "null");
})
test(function() {
  var script = document.createElement("script");
  script.setAttribute("event", "blah");
  script.setAttribute("for", "blah");
  script.event = undefined;
  script.htmlFor = undefined;
  assert_equals(script.event, "undefined");
  assert_equals(script.htmlFor, "undefined");
  assert_equals(script.getAttribute("event"), "undefined");
  assert_equals(script.getAttribute("for"), "undefined");
})
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/script-IDL-event-htmlfor.html"
}
```
