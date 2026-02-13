# html/syntax/speculative-parsing/expect-no-linked-resources/no-speculative-fetch.tentative.optional.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/expect-no-linked-resources/no-speculative-fetch.tentative.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Speculative parsing, expect-no-linked-resources Document-Policy</title>
<meta name="timeout" content="long">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/common/utils.js"></script>
<script>
  async function get_results(uuid) {
    const response = await fetch(`/html/syntax/speculative-parsing/expect-no-linked-resources/resources/stash.py?action=get&uuid=${uuid}`);
    return await response.text();
  }

  promise_test(async () => {
    const uuid = token();

    await test_driver.bless('Open a URL with expect-no-linked-resources Document-Policy');
    const popup = window.open(`/html/syntax/speculative-parsing/expect-no-linked-resources/resources/no-speculative-fetch.sub.html?pipe=sub&uuid=${uuid}`, '_blank');
    await new Promise(resolve => popup.addEventListener('load', resolve));

    const result = await get_results(uuid);
    assert_equals(result, '', 'speculative case fetched');
  }, `expect-no-linked-resources hint was ignored`);
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
  "source_name": "html/syntax/speculative-parsing/expect-no-linked-resources/no-speculative-fetch.tentative.optional.html"
}
```
