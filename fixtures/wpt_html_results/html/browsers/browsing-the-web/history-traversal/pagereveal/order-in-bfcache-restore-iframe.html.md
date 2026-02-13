# html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pagereveal event in iframe fires and in correct order on restoration from BFCache</title>
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
  scripts: ['/common/get-host-info.sub.js'],
  funcBeforeNavigation: async () => {
    // This function executes in pageA
    window.message = (data) => {
      return new Promise(resolve => {
        addEventListener('message', e => {
          if (data == undefined || data === e.data)
            resolve(e.data);
        });
      });
    }

    const frameLoaded = message('loaded');
    const iframe = document.createElement('iframe');
    iframe.src = `${get_host_info().OTHER_ORIGIN}/html/browsers/browsing-the-web/history-traversal/pagereveal/resources/iframe.html?restore`;
    document.body.appendChild(iframe);
    await frameLoaded;
  },
  funcAfterAssertion: async (pageA, pageB, t) => {
    let event_log = await pageA.execute_script(async () => {
      const event_log = message();
      document.querySelector('iframe').contentWindow.postMessage('getEventLog', '*');
      return await event_log;
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-bfcache-restore-iframe.html"
}
```
