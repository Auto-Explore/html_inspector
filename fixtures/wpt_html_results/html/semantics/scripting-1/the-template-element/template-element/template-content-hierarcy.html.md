# html/semantics/scripting-1/the-template-element/template-element/template-content-hierarcy.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/template-content-hierarcy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="author" title="Takayoshi Kochi" href="mailto:kochi@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<div id="parent">
  <template id="tmpl"><span>Happy Templating!</span></template>
</div>
<script>
test(() => {
  var parent = document.getElementById('parent');
  var tmpl = document.getElementById('tmpl');

  assert_equals(tmpl.innerHTML, '<span>Happy Templating!</span>');
  var span = tmpl.content.querySelector('span');

  // Hierarchy checks at various combinations.
  assert_throws_dom('HierarchyRequestError', () => {
    tmpl.content.appendChild(parent);
  }, 'Template content should throw if any of ancestor is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    tmpl.content.appendChild(tmpl);
  }, 'Template content should throw if its host is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    span.appendChild(parent);
  }, 'Template content child should throw if any of ancestor is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    span.appendChild(tmpl);
  }, 'Template content child should throw template\'s host is being appended.');
}, "Template content should throw when its ancestor is being appended.");

test(() => {
  var parent = document.getElementById('parent');
  var tmpl = document.getElementById('tmpl');

  assert_equals(tmpl.innerHTML, '<span>Happy Templating!</span>');
  var span = tmpl.content.querySelector('span');

  var tmpl_doc = tmpl.content.ownerDocument;
  assert_equals(tmpl.ownerDocument, document);
  assert_not_equals(tmpl_doc, document);

  var new_doc = document.implementation.createHTMLDocument();
  assert_not_equals(new_doc, document);
  assert_not_equals(new_doc, tmpl_doc);

  // Try moving tmpl.content to new_doc and check the results.
  const tmplContentAdoptResult = new_doc.adoptNode(tmpl.content);
  assert_equals(tmpl.content, tmplContentAdoptResult);
  assert_equals(tmpl.ownerDocument, document);
  assert_equals(tmpl.content.ownerDocument, new_doc);

  // Hierarchy checks at various combinations.
  assert_throws_dom('HierarchyRequestError', () => {
    tmpl.content.appendChild(parent);
  }, 'Template content should throw if any of ancestor is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    tmpl.content.appendChild(tmpl);
  }, 'Template content should throw if its host is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    span.appendChild(parent);
  }, 'Template content child should throw if any of ancestor is being appended.');
  assert_throws_dom('HierarchyRequestError', () => {
    span.appendChild(tmpl);
  }, 'Template content child should throw template\'s host is being appended.');

  // Sanity check: template.content before and after move.
  var tmpl_content_reference = tmpl.content;
  assert_equals(tmpl.content.firstChild, span,
                '<span> should be kept until it is removed, even after ' +
                'adopted to another document.');
  new_doc.body.appendChild(tmpl.content);
  assert_equals(tmpl.content.firstChild, null,
                '<span> should be removed from template content.');
  assert_equals(tmpl_content_reference, tmpl.content,
                'template.content should be identical before and after ' +
                'moving its children.');
}, 'Template content should throw exception when its ancestor in ' +
   'a different document but connected via host is being append.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 93,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 93,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/template-content-hierarcy.html"
}
```
