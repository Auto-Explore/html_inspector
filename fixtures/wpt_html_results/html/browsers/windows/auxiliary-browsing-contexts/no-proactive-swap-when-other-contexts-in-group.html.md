# html/browsers/windows/auxiliary-browsing-contexts/no-proactive-swap-when-other-contexts-in-group.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/no-proactive-swap-when-other-contexts-in-group.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>No proactive browsing context group changes when other contexts in group</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/browsers/browsing-the-web/remote-context-helper/resources/remote-context-helper.js"></script>

<body>
<script>

promise_test(async t => {
  const rcHelper = new RemoteContextHelper();
  const rcPopup1 = await rcHelper.addWindow(undefined, { target: 'my_popup' });

  await rcPopup1.executeScript(() => {
    window.intendedTarget1 = true;
  });
  assert_true(window.open('', 'my_popup').intendedTarget1,
              'lookup by name');

  // There is no security reason (e.g. COOP) to change browsing context groups
  // for this navigation. Some implementations perform BCG swaps for performance
  // reasons, but they cannot do so in this case, as this test harness page and
  // the popup are in the same browsing context group.
  const rcPopup2 = await rcPopup1.navigateToNew();

  // In order to find the popup by name, the popup and this opener must be in
  // the same browsing context group.
  await rcPopup2.executeScript(() => {
    window.intendedTarget2 = true;
  });
  assert_true(window.open('', 'my_popup').intendedTarget2,
              'lookup by name after navigation');
}, 'no proactive browsing context group change when other contexts in group');

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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/no-proactive-swap-when-other-contexts-in-group.html"
}
```
