# html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pagereveal event fires and in correct order on restoration from BFCache</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/browsing-the-web.html#updating-the-document">
<link rel="author" href="mailto:bokan@chromium.org">
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/browsers/browsing-the-web/back-forward-cache/resources/helper.sub.js"></script>
<script>
// runBfcacheTest opens a popup to pageA which navigates to pageB and then
// back, ensuring pageA is stored in the BFCache.
runBfcacheTest({
  funcBeforeNavigation: async () => {
    // This function executes in pageA

    // Wait for an animation frame to ensure the initial-load
    // `pagereveal` has already been fired so it doesn't get recorded
    // below.
    const raf = new Promise(resolve => requestAnimationFrame(resolve));
    await raf;

    window.event_log = [];
    let restored = false;

    function recordRafs() {
      requestAnimationFrame( () => {
        // Avoid recording animation frames until the page is restored from
        // BFCache since it's currently uncached. This test is interested only
        // in the behavior during restoration.
        if (restored)
          window.event_log.push('rAF');

        recordRafs();
      });
    }

    recordRafs();

    addEventListener('pageshow', (e) => {
      window.event_log.push('pageshow' + (e.persisted ? '.persisted' : ''));
      if (e.persisted)
        restored = true;
    });

    addEventListener('pagereveal', () => {
      window.event_log.push('pagereveal');
    });
  },
  funcAfterAssertion: async (pageA, pageB, t) => {
    let event_log = await pageA.execute_script(async () => {
      // Ensure at least one animation frame is produced to ensure
      // pagereveal must have fired.
      await new Promise(requestAnimationFrame);
      return window.event_log;
    });

    // Expect that the events seen are:
    // pageshow.persisted, pagereveal, rAF, rAF, rAF, ...
    assert_equals(event_log.slice(0, 3).toString(),
        'pageshow.persisted,pagereveal,rAF');
    for (let i = 3; i < event_log.length; ++i) {
      assert_equals(event_log[i], 'rAF',
          'All events following pagereveal should be animation frames');
    }
  },
  targetOrigin: originSameOrigin,
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore.html"
}
```
