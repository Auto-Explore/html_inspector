# html/browsers/browsing-the-web/back-forward-cache/timers.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/timers.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<link rel="help" href="https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers:fully-active">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="resources/helper.sub.js"></script>
<script>
// Timers should be paused when the Document is not fully active.
// This test is checking this by measuring the actual elapsed time for a timer
// started before a page is stored into BFCache, staying for a while in BFCache,
// and fired after the page is restored from BFCache.

const delayMain = 18000;
const delayBeforeForwardNavigation = 6000;
const delayBeforeBackNavigation = 5000;
// `delayBeforeForwardNavigation` and `delayBeforeBackNavigation` are set
// sufficiently large in order to distinguish the expected case from other
// scenarios listed in `funcAfterAssertion()`, and to allow some delays outside
// timers (e.g. due to communication between Windows). The additional delays
// can be large (e.g. ~4 seconds), so the delays above should be sufficiently
// large.

const startTime = performance.now();

runBfcacheTest({
  funcBeforeNavigation: async (delayMain, delayBeforeForwardNavigation) => {
    // Set `promiseMainTimer` that is resolved after a timeout of `delayMain`
    // ms.
    window.promiseMainTimer = new Promise(resolve => {
      setTimeout(resolve, delayMain);
    });
    // Then navigate to another page after `delayBeforeForwardNavigation` ms.
    await new Promise(resolve =>
        setTimeout(resolve, delayBeforeForwardNavigation));
  },
  argsBeforeNavigation: [delayMain, delayBeforeForwardNavigation],
  funcBeforeBackNavigation: async (delayBeforeBackNavigation) => {
    // Back navigate after `delayBeforeBackNavigation` ms.
    await new Promise(resolve =>
        setTimeout(resolve, delayBeforeBackNavigation));
  },
  argsBeforeBackNavigation: [delayBeforeBackNavigation],
  funcAfterAssertion: async (pageA) => {
    // Wait for `promiseMainTimer` resolution and check its timing.
    await pageA.execute_script(() => window.promiseMainTimer);
    const actualDelay = performance.now() - startTime;

    if (actualDelay >= delayMain + delayBeforeBackNavigation +
                       delayBeforeForwardNavigation) {
      assert_unreached(
        "The timer is fired too late. " +
        "Maybe the timer is reset when restored from BFCache and " +
        "waits from the beginning again");
    } else if (actualDelay >= delayMain + delayBeforeBackNavigation) {
      // Expected: The timer is paused when the page is in BFCache.
    } else if (actualDelay >= delayMain) {
      assert_unreached(
        "The timer is fired too early. " +
        "Maybe the time isn't paused when the page is in BFCache");
    } else {
      assert_unreached(
        "The timer is fired too early, even earlier than delayMain.");
    }
  }
}, 'Timers should be paused when the page is in BFCache');
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/timers.html"
}
```
