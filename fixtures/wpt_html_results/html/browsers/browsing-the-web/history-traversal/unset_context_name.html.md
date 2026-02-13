# html/browsers/browsing-the-web/history-traversal/unset_context_name.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/unset_context_name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>window.name is reset after navigating to a different origin</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script>

async function pollResultAndCheck(t, id, expected) {
  const stashURL = new URL("resources/unset_context_name_stash.py", location);
  stashURL.searchParams.set('id', id);

  let res = "NONE";
  while (res == "NONE") {
    await new Promise(resolve => { t.step_timeout(resolve, 100); });

    const response = await fetch(stashURL);
    res = await response.text();
  }
  if (res !== expected) {
    assert_unreached('Stash result does not equal expected result.')
  }
}

promise_test(async t => {
  const id = token();

  window.open(`resources/unset_context_name-1.sub.html?set=${id}|navigate|report=${id}|close`, "_blank", "noopener");
  await pollResultAndCheck(t, id, "");
}, "Window.name is reset after navigating to a different origin");

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
  "source_name": "html/browsers/browsing-the-web/history-traversal/unset_context_name.html"
}
```
