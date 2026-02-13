# html/browsers/browsing-the-web/history-traversal/browsing_context_name-0.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/browsing_context_name-0.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<iframe id="test"></iframe>
<script>
var opener = window.opener;
var t = opener.t;
var f = document.getElementById("test");
var l = opener.document.getElementById("step_log");

log = function(t) {l.textContent += ("\n" + t)}
var navigated = false;
var steps = [
  () => {
    opener.assert_equals(history.length, 1, "first history.length");
    f.src = "browsing_context_name-1.html";
  },
  () => {
    navigated = true;
    opener.assert_equals(f.contentWindow.name, "test", "Initial load");
    opener.assert_equals(history.length, 1, "second history.length");
    setTimeout(next, 0);
  },
  () => {
    opener.assert_equals(f.contentWindow.name, "test", "Initial load");
    opener.assert_equals(history.length, 1, "third history.length");
    f.src = "browsing_context_name-2.html"
  },
  () => {
    opener.assert_equals(f.contentWindow.name, "test1");
    opener.assert_equals(history.length, 2, "fourth history.length");
    setTimeout(next, 0);
  },
  () => {
    opener.assert_equals(f.contentWindow.name, "test1");
    opener.assert_equals(history.length, 2, "fifth history.length");
    history.back();
  },
  () => {
    opener.assert_equals(history.length, 2, "sixth history.length");
    opener.assert_equals(f.contentWindow.name, "test1", "After navigation");
    setTimeout(next, 0);
  },
  () => {
    opener.assert_equals(history.length, 2, "seventh history.length");
    opener.assert_equals(f.contentWindow.name, "test1", "After navigation");
    t.done();
  }
].map((x, i) => t.step_func(() => {log("Step " + (i+1)); x()}));

next = () => steps.shift()();

onload = () => {
  log("page load");
  f.onload = () => {
    log("iframe onload");
    next();
  };
  setTimeout(next, 0);
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 18,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/browsing_context_name-0.html"
}
```
