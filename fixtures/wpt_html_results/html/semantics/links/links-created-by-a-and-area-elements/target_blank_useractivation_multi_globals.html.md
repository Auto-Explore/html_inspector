# html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation_multi_globals.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation_multi_globals.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset=utf-8>
<title>Multi-globals: which userActivation should be consumed when opening a target=_blank link?</title>
<link rel="help" href="https://html.spec.whatwg.org/#following-hyperlinks-2">
<link rel="help" href="https://html.spec.whatwg.org/#the-rules-for-choosing-a-navigable">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
</head>
<body>
<script>

const testUrl = "target-blank-useractivation.html";

function waitForNewWindow(aBroadcastChannelId) {
  return new Promise(resolve => {
    let channel = new BroadcastChannel(aBroadcastChannelId);
    channel.addEventListener("message", (e) => {
      assert_equals(e.data, "ready");
      resolve(channel);
    }, {once: true});
  });
}

["a", "area"].forEach(tag => {
  promise_test(async (t) => {
    await test_driver.bless('active main test window to open popup test window');
    let popupPromise = waitForNewWindow("popup");
    let popup = window.open(`support/${testUrl}?popup`);
    t.add_cleanup(() => { popup.close(); });
    await popupPromise;

    const broadcastChannelId = `${tag}_click_script`;
    const link = popup.document.querySelector(tag);
    link.href = `${testUrl}?${broadcastChannelId}`;

    await test_driver.bless('active main test window again');
    assert_true(navigator.userActivation.isActive, 'test window should have user activation');

    await test_driver.bless('active popup test window', () => {}, popup);
    assert_true(popup.navigator.userActivation.isActive, 'popup should have user activation');

    let newWindowPromise = waitForNewWindow(broadcastChannelId);
    link.click();
    let newWindowChannel = await newWindowPromise;
    t.add_cleanup(() => {  newWindowChannel.postMessage("close"); });

    assert_true(navigator.userActivation.isActive, 'main test window should still have user activation');
    assert_false(popup.navigator.userActivation.isActive, 'popup test window should not have user activation');
  }, `<${tag} target=_blank">.click()`);
});
</script>
</body>
</html>
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
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation_multi_globals.html"
}
```
