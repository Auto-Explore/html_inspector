# html/cross-origin-embedder-policy/iframe-history-none-require-corp.https.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/iframe-history-none-require-corp.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta name="timeout" content="long">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/utils.js></script
<script src="/common/get-host-info.sub.js"></script>
<script>

promise_test(async test => {
  // TODO(arthursonzogni): Consider switching toward another message passing
  // API like:
  // /common/dispatcher/dispatcher.js
  const bc = new BroadcastChannel(token());
  const futureMessage = () => {
    return new Promise(resolve => {
      bc.onmessage = event => resolve(event.data);
    });
  };

  const prefix = document.URL.substr(0, document.URL.lastIndexOf('/'))
  const attribute = `?channelName=${bc.name}`;
  const url_coep_none =
    prefix + "/resources/navigate-none.sub.html" + attribute;
  const url_coep_require_corp =
    prefix + "/resources/navigate-require-corp.sub.html" + attribute;

  const w = window.open();
  test.add_cleanup(() => w.close());

  // Navigate to COEP:unsafe-none.
  w.location.href = url_coep_none;
  assert_equals(await futureMessage(), "loaded");
  assert_equals(w.location.href, url_coep_none);

  // For unknown reasons so far. Waiting in between the different navigations
  // avoids flakes.
  await new Promise(resolve => test.step_timeout(resolve, 1000));

  // Navigate to COEP:require-corp.
  w.location.href = url_coep_require_corp;
  assert_equals(await futureMessage(), "loaded");
  assert_equals(w.location.href, url_coep_require_corp);

  // For unknown reasons so far. Waiting in between the different navigations
  // avoids flakes.
  await new Promise(resolve => test.step_timeout(resolve, 1000));

  // Navigate back to COEP:unsafe-none, using the history API.
  // Note: `url_coep_none` already take the BFCache into account.
  w.history.back();
  assert_equals(await futureMessage(), "loaded");
  assert_equals(w.location.href, url_coep_none);
}, `"none" top-level: navigating a frame back from "require-corp" should succeed`);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 36,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.end_tag_with_attrs",
      "message": "End tag had attributes.",
      "severity": "Warning",
      "span": {
        "byte_end": 220,
        "byte_start": 168,
        "col": 30,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 229,
        "byte_start": 220,
        "col": 44,
        "line": 5
      }
    },
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
  "source_name": "html/cross-origin-embedder-policy/iframe-history-none-require-corp.https.html"
}
```
