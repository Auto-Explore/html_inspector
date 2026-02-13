# html/webappapis/scripting/events/contextmenu-event-manual.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/contextmenu-event-manual.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>HTML contextmenu event is a MouseEvent</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <style>#contextmenutarget { width: 100px; height: 100px; background-color: red; }</style>
  </head>
  <body>
    <div id='contextmenutarget'>Trigger context menu in this box.</div>
    <div id="log"></div>
    <script type="text/javascript">
var t = async_test('contextmenu event generated from user action is MouseEvent');
document.querySelector("#contextmenutarget").addEventListener('contextmenu', t.step_func(function (e) {
    assert_equals(e.constructor, window.MouseEvent);
    document.querySelector("#contextmenutarget").style.backgroundColor = "green";
    t.done();
}));
    </script>
  </body>
</html>
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
        "byte_end": 449,
        "byte_start": 418,
        "col": 5,
        "line": 12
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
  "source_name": "html/webappapis/scripting/events/contextmenu-event-manual.htm"
}
```
