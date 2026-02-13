# html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin_3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin_3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Restoring window.name on cross-origin history traversal</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<pre id="step_log"></pre>
<iframe id="test"></iframe>
<script>
var t = async_test();
var f = document.getElementById("test");
var l = document.getElementById("step_log");

log = function(t) {l.textContent += ("\n" + t)}

var steps = [
  function() {f.src = "browsing_context_name-1.html"},
  function() {
                assert_equals(f.contentWindow.name, "test", "Initial load");
                setTimeout(next, 0);
              },
  function() {f.src = "browsing_context_name-3.html"},
  function() {
                assert_equals(f.contentWindow.name, "test3", "After navigation 1");
                setTimeout(next, 0);
              },
  function() {f.src = f.src.replace("http://", "http://www.").replace("browsing_context_name-1", "browsing_context_name-2");},
  function() {f.src = f.src.replace("http://www.", "http://").replace("browsing_context_name-2", "browsing_context_name-4");},
  function() {
                assert_equals(f.contentWindow.name, "test3", "After navigation 2");
                history.go(-3); setTimeout(next, 500)
             },
  function() {
               assert_equals(f.contentWindow.name, "test3", "After navigation 3");
               t.done();
             }
].map(function(x) {return t.step_func(function() {log("Step " + step + " " + f.contentWindow.location); x()})});

var step = 0;
next = t.step_func(function() {steps[step++]()});

f.onload=next;

onload = function() { setTimeout(next, 0); };
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin_3.html"
}
```
