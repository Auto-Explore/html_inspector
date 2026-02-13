# html/webappapis/scripting/events/event-handler-sourcetext.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-sourcetext.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test the sourceText of event handlers</title>
<link rel="help" href="https://github.com/whatwg/html/issues/5500">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  const el = document.createElement("div");
  el.setAttribute("onclick", "foo");
  assert_equals(el.onclick.toString(), "function onclick(event) {\nfoo\n}");
}, "non-error event handler");

test(() => {
  const el = document.createElement("div");
  el.setAttribute("onerror", "foo");
  assert_equals(el.onerror.toString(), "function onerror(event) {\nfoo\n}");
}, "error event handler not on body");

test(() => {
  const el = document.createElement("body");
  el.setAttribute("onerror", "foo");
  assert_equals(el.onerror.toString(), "function onerror(event, source, lineno, colno, error) {\nfoo\n}");
}, "error event handler on disconnected body");

test(() => {
  const el = document.createElement("frameset");
  el.setAttribute("onerror", "foo");
  assert_equals(el.onerror.toString(), "function onerror(event, source, lineno, colno, error) {\nfoo\n}");
}, "error event handler on disconnected frameset");

test(() => {
  document.body.setAttribute("onerror", "foo");
  assert_equals(window.onerror.toString(), "function onerror(event, source, lineno, colno, error) {\nfoo\n}");
}, "error event handler on connected body, reflected to Window");
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
  "source_name": "html/webappapis/scripting/events/event-handler-sourcetext.html"
}
```
