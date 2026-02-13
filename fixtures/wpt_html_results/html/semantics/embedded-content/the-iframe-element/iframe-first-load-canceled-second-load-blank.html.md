# html/semantics/embedded-content/the-iframe-element/iframe-first-load-canceled-second-load-blank.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-first-load-canceled-second-load-blank.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>

// Check about:blank navigation is asynchronous, even if the initial navigation
// is canceled.
promise_test(async test => {
  // Wait for document.body to exist, before appending the <iframe>.
  await new Promise(resolve => window.onload = resolve);

  // The initial navigation in this new iframe will result in a 204 No Content.
  // As a result the navigation will be canceled. The <iframe> will stay on the
  // initial empty document.
  const iframe = document.createElement("iframe");
  iframe.src = "/common/blank.html?pipe=status(204)"
  document.body.appendChild(iframe);

  // The navigation in the iframe will be canceled. There are no good ways to
  // detect when it will happen. Anyway, any additional navigation must be
  // asynchronous. To test what happens when there are no pending navigation,
  // waiting one second should be enough, most of the time.
  await new Promise(resolve => test.step_timeout(resolve, 1000));

  let load_event_fired = false;
  const load_event_promise = new Promise(resolve => {
    iframe.onload = () => {
      load_event_fired = true;
      resolve();
    };
  });
  iframe.src = "about:blank";
  assert_equals(load_event_fired, false,
    "about:blank must not commit synchronously");
  await load_event_promise;
}, "about:blank navigation is asynchronous, even if the initial one is " +
  "canceled.");

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
        "byte_end": 40,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-first-load-canceled-second-load-blank.html"
}
```
