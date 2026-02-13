# html/browsers/windows/nested-browsing-contexts/window-top.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/window-top.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>window.top</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_equals(window, top)
}, "Top level browsing context");

function step_func(test) {
  return function (top_pointer) {
    test.step(function() {assert_equals(top_pointer, window);})
    test.done();
  }
}

var t1 = async_test("One nested iframe");
t1.step(function() {
  var iframe = document.createElement("iframe");
  //iframe.src = "data:text/html,"

  iframe.onload = t1.step_func(
    function() {
      var doc = iframe.contentDocument;
      iframe.contentWindow.test_func = step_func(t1);

      var script = doc.createElement("script")
      script.textContent = "test_func(top);"
      doc.body.appendChild(script);
    });
    document.body.appendChild(iframe);
});

var t2 = async_test("Two nested iframes");
t2.step(function() {
  var iframe = document.createElement("iframe");
  //iframe.src = "data:text/html,"

  iframe.onload = t2.step_func(
    function() {
      var doc = iframe.contentDocument;
      iframe2 = document.createElement("iframe");
      //iframe2.src = "data:text/html,"
      // Workaround for https://bugzilla.mozilla.org/show_bug.cgi?id=1229707
      iframe2.src = '/common/blank.html';

      iframe2.onload = t2.step_func(
        function() {
          var doc2 = iframe2.contentDocument;

          iframe2.contentWindow.test_func = step_func(t2);

          var script = doc2.createElement("script")
          script.textContent = "test_func(top);"
          doc2.body.appendChild(script);
        });
      doc.body.appendChild(iframe2);
   });

  document.body.appendChild(iframe);
});

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
  "source_name": "html/browsers/windows/nested-browsing-contexts/window-top.html"
}
```
