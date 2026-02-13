# html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/no-help-cursor-on-links.historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/no-help-cursor-on-links.historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>link with rel="help" cursor tests</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<link rel="help" href="https://html.spec.whatwg.org/#phrasing-content-3">
<link rel="help" href="https://github.com/whatwg/html/pull/3902">

<div id="log"></div>

<a href="/common/blank.html?unvisited" rel="help" id="unvisited">unvisited</a>
<a href="/common/blank.html?willbevisited" rel="help" id="willbevisited">will be visited</a>

<script>
"use strict";


test(() => {
  const el = document.querySelector("#unvisited");
  const style = window.getComputedStyle(el);

  assert_equals(style.cursor, "pointer");
},"Unvisited help links must have pointer cursor, not help cursor");


// This test is kind of dubious. Browsers don't allow you to distinguish visited and unvisited links
// from script, for privacy reasons. So we can't really be sure that loading the iframe would make
// the link count as visited. Manually running this test turns the link purple in some browsers,
// but leaves it blue in others. Even then it's not clear whether it turned purple before or after
// the onload; this test assumes that once the iframe onload fires, it counts as visited, which
// may not be justified even in the purple-turning browsers.
//
// Still, the test doesn't really hurt. At worst it's redundant with the above.
//
// If someone comes up with a better way of testing this (i.e. something that truly guarantees that
// the link will count as "visited" for UA stylesheet purposes), then please submit a PR.
async_test(t => {
  const el = document.querySelector("#willbevisited");

  const iframe = document.createElement("iframe");
  iframe.src = el.href;
  iframe.onload = t.step_func_done(() => {
    const style = window.getComputedStyle(el);
    assert_equals(style.cursor, "pointer");
  });

  document.body.appendChild(iframe);
}, "Visited help links must have pointer cursor, not help cursor");
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
  "source_name": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/no-help-cursor-on-links.historical.html"
}
```
