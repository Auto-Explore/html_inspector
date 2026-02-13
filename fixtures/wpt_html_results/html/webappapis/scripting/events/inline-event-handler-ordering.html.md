# html/webappapis/scripting/events/inline-event-handler-ordering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/inline-event-handler-ordering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Inline event handlers retain their ordering even when invalid</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
setup({ allow_uncaught_exception: true });
var events = [];

test(function() {
  events = [];
  var e = document.createElement("div");
  document.body.appendChild(e);
  e.addEventListener("click", function() { events.push("ONE") });
  e.setAttribute("onclick", "window.open(");
  e.addEventListener("click", function() { events.push("THREE") });
  // Try to compile the event handler.
  e.onclick;
  e.setAttribute("onclick", "events.push('TWO')");
  e.dispatchEvent(new Event("click"));
  var expected_events = ["ONE", "TWO", "THREE"];
  assert_array_equals(events, expected_events);
}, "Inline event handlers retain their ordering when invalid and force-compiled");

test(function() {
  events = [];
  var e = document.createElement("div");
  document.body.appendChild(e);
  e.addEventListener("click", function() { events.push("ONE") });
  e.setAttribute("onclick", "window.open(");
  e.addEventListener("click", function() { events.push("THREE") });
  e.dispatchEvent(new Event("click"));
  e.setAttribute("onclick", "events.push('TWO')");
  e.dispatchEvent(new Event("click"));
  var expected_events = ["ONE", "THREE", "ONE", "TWO", "THREE"];
  assert_array_equals(events, expected_events);
}, "Inline event handlers retain their ordering when invalid and force-compiled via dispatch");

test(function() {
  events = [];
  var e = document.createElement("div");
  document.body.appendChild(e);
  e.addEventListener("click", function() { events.push("ONE") });
  e.setAttribute("onclick", "window.open(");
  e.addEventListener("click", function() { events.push("THREE") });
  e.setAttribute("onclick", "events.push('TWO')");
  e.dispatchEvent(new Event("click"));
  var expected_events = ["ONE", "TWO", "THREE"];
  assert_array_equals(events, expected_events);
}, "Inline event handlers retain their ordering when invalid and lazy-compiled");
</script>
</body>
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
  "source_name": "html/webappapis/scripting/events/inline-event-handler-ordering.html"
}
```
