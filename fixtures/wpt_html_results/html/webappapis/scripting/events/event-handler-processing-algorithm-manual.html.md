# html/webappapis/scripting/events/event-handler-processing-algorithm-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Event handlers processing algorithm: manual tests</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-event-handler-processing-algorithm">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<style>
  div[id^="d"] {
    width: 100px;
    height: 100px;
    background-color: blue;
  }
</style>

<div id="log"></div>

<p>Mouseover these four divs</p>

<div id="d1"></div>
<div id="d2"></div>

<div id="d3" onmouseover="return false"></div>
<div id="d4" onmouseover="return true"></div>

<script>
"use strict";
async_test(t => {
  const div = document.querySelector("#d1");

  div.onmouseover = t.step_func(() => false);
  div.addEventListener("mouseover", t.step_func_done(e => {
    assert_equals(e.defaultPrevented, true);
  }));
}, "Listener added via JavaScript, returns false: cancels the event");

async_test(t => {
  const div = document.querySelector("#d2");

  div.onmouseover = t.step_func(() => true);
  div.addEventListener("mouseover", t.step_func_done(e => {
    assert_equals(e.defaultPrevented, false);
  }));
}, "Listener added via JavaScript, returns true: does not cancel the event");

async_test(t => {
  const div = document.querySelector("#d3");

  div.addEventListener("mouseover", t.step_func_done(e => {
    assert_equals(e.defaultPrevented, true);
  }));
}, "Listener added via markup, returns false: cancels the event");

async_test(t => {
  const div = document.querySelector("#d4");

  div.addEventListener("mouseover", t.step_func_done(e => {
    assert_equals(e.defaultPrevented, false);
  }));
}, "Listener added via markup, returns true: does not cancel the event");
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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-manual.html"
}
```
