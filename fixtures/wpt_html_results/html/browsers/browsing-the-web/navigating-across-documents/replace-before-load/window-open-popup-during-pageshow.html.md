# html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/window-open-popup-during-pageshow.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/window-open-popup-during-pageshow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>

<body>
<script>
"use strict";
promise_test(async t => {
  // Generating a new one instead of hard-coding makes running tests manually a bit easier.
  const windowName = token();

  const code = `
    window.onpageshow = () => opener.navigateMe();
    opener.postMessage("arrived at start URL", "*");
  `;

  const startURL = "resources/slow-code-injector.html?pipe=sub(none)&code=" + encodeURIComponent(code);
  const absoluteStartURL = (new URL(startURL, location.href)).href;

  const afterReplacementURL = "resources/message-opener.html";
  const absoluteAfterReplacementURL = (new URL(afterReplacementURL, location.href)).href;

  window.navigateMe = () => {
    window.open(absoluteAfterReplacementURL, windowName);
  };

  // First message sent is ignored; we only check it after navigating back.
  const w = window.open(startURL, windowName);
  t.add_cleanup(() => w.close());

  // Wait to get past any initial about:blank
  while (true) {
    if (w.location.href === absoluteStartURL) {
      break;
    }
    await new Promise(r => t.step_timeout(r, 0));
  }

  assert_equals(w.onloadFired, undefined, "onload must not yet have fired");
  assert_equals(w.history.length, 1, "history.length for the opened window must start at 1");

  await new Promise(r => {
    window.addEventListener("message", t.step_func(e => {
      if (e.data === "ready") {
        resolve();
      }
    }));
  });

  assert_equals(w.history.length, 2, "history.length must increase");
  assert_equals(w.location.href, absoluteAfterReplacementURL);

  const promise = new Promise(resolve => {
    window.addEventListener("message", t.step_func(e => {
      assert_equals(e.data, "arrived at start URL");
      resolve();
    }));
  });

  w.history.back();

  await promise;
  assert_equals(w.location.href, absoluteStartURL, "1 second after attempting to go back, it indeed went back");
}, "No replace before load, triggered by window.open() on a non-_self window");
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/window-open-popup-during-pageshow.html"
}
```
