# html/infrastructure/urls/base-url/document-base-url-changes-about-srcdoc-2.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/base-url/document-base-url-changes-about-srcdoc-2.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>

<iframe src='about:blank'></iframe>

<script>
// If document.open's behavior is modified to remove the url-rewriting behavior,
// then this test can be deleted (https://github.com/whatwg/html/issues/3989).
setup({ explicit_done: true });

// This function is called by directly by the child iframe (above), with the
// child iframe's document. The child iframe's document contains an about:srcdoc iframe.
window.start = childDocument => {
  const grandchildDocument =
    childDocument.getElementById('foo').contentDocument;

  test(() => {
    assert_equals(childDocument.URL, 'about:blank',
      'Child document starting URL');
    assert_equals(grandchildDocument.URL, 'about:srcdoc',
      'Grandchild document starting URL');
    const originalChildBaseURL = childDocument.baseURI;

    grandchildDocument.open("", "");
    // Verify that the document.open() trick worked: the grandchild should now
    // have the same url as the child, and have inherited the child's base url.
    assert_equals(grandchildDocument.URL, 'about:blank',
      'Grandchild document after document.open() trick');
    assert_equals(grandchildDocument.baseURI, originalChildBaseURL,
      'Grandchild base URL must match child base URL');

    // Give child a new base URL.
    const baseElement = childDocument.createElement('base');
    baseElement.href = get_host_info().REMOTE_ORIGIN;
    childDocument.head.append(baseElement);

    // Verify that changing the child's base url succeeded and did not affect
    // the grandchild's base url.
    const newChildBaseURL = childDocument.baseURI;
    assert_equals(grandchildDocument.URL, 'about:blank',
      'Grandchild document after child gets new base URL');
    assert_not_equals(newChildBaseURL, originalChildBaseURL,
      'Child base URL must change');
    assert_equals(grandchildDocument.baseURI, originalChildBaseURL,
      'Grandchild base URL must not change');
  });

  done();
};

let subframe_doc = document.querySelector('iframe').contentDocument;
subframe_doc.body.innerHTML = '<iframe srcdoc="foo" id="foo"></iframe>';
promise_test(async t => {
  const grandchildIframe = subframe_doc.querySelector('iframe');
  await new Promise(resolve => {
    grandchildIframe.onload = resolve;
  });

  assert_equals(grandchildIframe.contentDocument.URL, 'about:srcdoc');

  let script = subframe_doc.createElement('script');
  script.innerHTML = 'parent.start(document);';
  subframe_doc.body.appendChild(script);
}, "wrapper promise test for timeout.");
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
  "source_name": "html/infrastructure/urls/base-url/document-base-url-changes-about-srcdoc-2.https.html"
}
```
