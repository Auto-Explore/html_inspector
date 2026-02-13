# html/dom/aria-element-reflection.html

Counts:
- errors: 7
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/aria-element-reflection.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Element Reflection for aria-activedescendant and aria-errormessage</title>
    <link rel=help href="https://whatpr.org/html/3917/common-dom-interfaces.html#reflecting-content-attributes-in-idl-attributes:element">
    <link rel="author" title="Meredith Lane" href="meredithl@chromium.org">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <div id="activedescendant" aria-activedescendant="x"></div>

  <div id="parentListbox" role="listbox" aria-activedescendant="i1">
    <div role="option" id="i1">Item 1</div>
    <div role="option" id="i2">Item 2</div>
  </div>

  <script>
  test(function(t) {
    assert_equals(activedescendant.ariaActiveDescendantElement, null,
                  "invalid ID for relationship returns null");

    // Element reference should be set if the content attribute was included.
    assert_equals(parentListbox.getAttribute("aria-activedescendant"), "i1", "check content attribute after parsing.");
    assert_equals(parentListbox.ariaActiveDescendantElement, i1, "check idl attribute after parsing.");
    assert_equals(parentListbox.ariaActiveDescendantElement, parentListbox.ariaActiveDescendantElement, "check idl attribute caching after parsing.");

    // If we set the content attribute, the element reference should reflect this.
    parentListbox.setAttribute("aria-activedescendant", "i2");
    assert_equals(parentListbox.ariaActiveDescendantElement, i2, "setting the content attribute updates the element reference.");
    assert_equals(parentListbox.ariaActiveDescendantElement, parentListbox.ariaActiveDescendantElement, "check idl attribute caching after update.");

    // Setting the element reference should set the empty string in the content attribute.
    parentListbox.ariaActiveDescendantElement = i1;
    assert_equals(parentListbox.ariaActiveDescendantElement, i1, "getter should return the right element reference.");
    assert_equals(parentListbox.getAttribute("aria-activedescendant"), "", "content attribute should be empty.");

    // Both content and IDL attribute should be nullable.
    parentListbox.ariaActiveDescendantElement = null;
    assert_equals(parentListbox.ariaActiveDescendantElement, null);
    assert_false(parentListbox.hasAttribute("aria-activedescendant"));
    assert_equals(parentListbox.getAttribute("aria-activedescendant"), null, "nullifying the idl attribute removes the content attribute.");

    // Setting content attribute to non-existent or non compatible element should nullify the IDL attribute.
    // Reset the element to an existant one.
    parentListbox.setAttribute("aria-activedescendant", "i1");
    assert_equals(parentListbox.ariaActiveDescendantElement, i1, "reset attribute.");

    parentListbox.setAttribute("aria-activedescendant", "non-existent-element");
    assert_equals(parentListbox.getAttribute("aria-activedescendant"), "non-existent-element");
    assert_equals(parentListbox.ariaActiveDescendantElement, null,"non-DOM content attribute should null the element reference");
  }, "aria-activedescendant element reflection");
  </script>

  <div id="parentListbox2" role="listbox" aria-activedescendant="option1">
    <div role="option" id="option1">Item 1</div>
    <div role="option" id="option2">Item 2</div>
  </div>

  <script>
  test(function(t) {
    const option1 = document.getElementById("option1");
    const option2 = document.getElementById("option2");
    assert_equals(parentListbox2.ariaActiveDescendantElement, option1);
    option1.removeAttribute("id");
    option2.setAttribute("id", "option1");
    const option2Duplicate = document.getElementById("option1");
    assert_equals(option2, option2Duplicate);

    assert_equals(parentListbox2.ariaActiveDescendantElement, option2);
  }, "If the content attribute is set directly, the IDL attribute getter always returns the first element whose ID matches the content attribute.");
  </script>

  <div id="blankIdParent" role="listbox">
    <div role="option" id="multiple-id"></div>
    <div role="option" id="multiple-id"></div>
  </div>

  <script>
  test(function(t) {
    // Get second child of parent. This violates the setting of a reflected element
    // as it will not be the first child of the parent with that ID, which should
    // result in an empty string for the content attribute.
    blankIdParent.ariaActiveDescendantElement = blankIdParent.children[1];
    assert_true(blankIdParent.hasAttribute("aria-activedescendant"));
    assert_equals(blankIdParent.getAttribute("aria-activedescendant"), "");
    assert_equals(blankIdParent.ariaActiveDescendantElement, blankIdParent.children[1]);
  }, "Setting the IDL attribute to an element which is not the first element in DOM order with its ID causes the content attribute to be an empty string");
  </script>

  <div id="outerContainer">
    <p id="lightParagraph">Hello world!</p>
    <span id="shadowHost">
    </span>
  </div>

  <script>
  test(function(t) {
    const shadow = shadowHost.attachShadow({mode: "open"});
    const link = document.createElement("a");
    shadow.appendChild(link);

    assert_equals(lightParagraph.ariaActiveDescendantElement, null);

    // The given element crosses a shadow dom boundary, so it cannot be
    // set as an element reference.
    lightParagraph.ariaActiveDescendantElement = link;
    assert_equals(lightParagraph.ariaActiveDescendantElement, null);

    // The given element crosses a shadow dom boundary (upwards), so
    // can be used as an element reference, but the content attribute
    // should reflect the empty string.
    link.ariaActiveDescendantElement = lightParagraph;
    assert_equals(link.ariaActiveDescendantElement, lightParagraph);
    assert_equals(link.getAttribute("aria-activedescendant"), "");
  }, "Setting an element reference that crosses into a shadow tree is disallowed, but setting one that is in a shadow inclusive ancestor is allowed.");
  </script>

  <input id="startTime" ></input>
  <span id="errorMessage">Invalid Time</span>

  <script>
  test(function(t) {
    startTime.ariaErrorMessageElements = [errorMessage];
    assert_equals(startTime.getAttribute("aria-errormessage"), "");
    assert_array_equals(startTime.ariaErrorMessageElements, [errorMessage]);

    startTime.ariaErrorMessageElements = [];
    assert_array_equals(startTime.ariaErrorMessageElements, []);
    assert_equals(startTime.getAttribute("aria-errormessage"), "");

    startTime.setAttribute("aria-errormessage", "errorMessage");
    assert_array_equals(startTime.ariaErrorMessageElements, [errorMessage]);

  }, "aria-errormessage");

  test(function (t) {
      assert_false('ariaErrorMessageElement' in startTime);
  }, 'ariaErrorMessageElement is not defined')

  </script>

  <label>
    Password:
    <input id="passwordField" type="password" aria-details="pw">
  </label>

  <ul>
    <li id="listItem1">First description.</li>
    <li id="listItem2">Second description.</li>
  </ul>

  <script>

  test(function(t) {
    assert_array_equals(passwordField.ariaDetailsElements, []);
    passwordField.ariaDetailsElements = [ listItem1 ];
    assert_equals(passwordField.getAttribute("aria-details"), "");
    assert_array_equals(passwordField.ariaDetailsElements, [ listItem1 ]);

    passwordField.ariaDetailsElements = [ listItem2 ];
    assert_equals(passwordField.getAttribute("aria-details"), "");
    assert_array_equals(passwordField.ariaDetailsElements, [ listItem2 ]);
  }, "aria-details");
  </script>

  <div id="deletionParent" role="listbox" aria-activedescendant="contentAttrElement">
    <div role="option" id="contentAttrElement">Item 1</div>
    <div role="option" id="idlAttrElement">Item 2</div>
  </div>

  <script>

  test(function(t) {
    const contentAttrElement = document.getElementById("contentAttrElement");
    const idlAttrElement = document.getElementById("idlAttrElement");

    assert_equals(deletionParent.getAttribute("aria-activedescendant"), "contentAttrElement");
    assert_equals(deletionParent.ariaActiveDescendantElement, contentAttrElement);

    // Deleting an element set via the content attribute.
    deletionParent.removeChild(contentAttrElement);
    assert_equals(deletionParent.getAttribute("aria-activedescendant"), "contentAttrElement");

    // As it was not explitly set, the attr-associated-element is computed from the content attribute,
    // and since descendant1 has been removed from the DOM, it is not valid.
    assert_equals(deletionParent.ariaActiveDescendantElement, null);

    // Deleting an element set via the IDL attribute.
    deletionParent.ariaActiveDescendantElement = idlAttrElement;
    assert_equals(deletionParent.getAttribute("aria-activedescendant"), "");

    deletionParent.removeChild(idlAttrElement);
    assert_equals(deletionParent.ariaActiveDescendantElement, null);

    // The content attribute is still empty.
    assert_equals(deletionParent.getAttribute("aria-activedescendant"), "");
  }, "Deleting a reflected element should return null for the IDL attribute and the content attribute will be empty.");
  </script>

  <div id="parentNode" role="listbox" aria-activedescendant="changingIdElement">
    <div role="option" id="changingIdElement">Item 1</div>
    <div role="option" id="persistantIDElement">Item 2</div>
  </div>

  <script>
  test(function(t) {
    const changingIdElement = document.getElementById("changingIdElement");
    assert_equals(parentNode.ariaActiveDescendantElement, changingIdElement);

    // Modify the id attribute.
    changingIdElement.setAttribute("id", "new-id");

    // The content attribute still reflects the old id, and we expect the
    // Element reference to be null as there is no DOM node with id "original"
    assert_equals(parentNode.getAttribute("aria-activedescendant"), "changingIdElement");
    assert_equals(parentNode.ariaActiveDescendantElement, null, "Element set via content attribute with a changed id will return null on getting");

    parentNode.ariaActiveDescendantElement = changingIdElement;
    assert_equals(parentNode.getAttribute("aria-activedescendant"), "");
    assert_equals(parentNode.ariaActiveDescendantElement, changingIdElement);

    // The explicitly set element takes precendance over the content attribute.
    // This means that we still return the same element reference, but the
    // content attribute is empty.
    changingIdElement.setAttribute("id", "newer-id");
    assert_equals(parentNode.ariaActiveDescendantElement, changingIdElement, "explicitly set element is still present even after the id has been changed");
    assert_equals(parentNode.getAttribute("aria-activedescendant"), "", "content attribute is empty.");
  }, "Changing the ID of an element doesn't lose the reference.");
  </script>

  <!-- TODO(chrishall): change naming scheme to inner/outer -->
  <div id="lightParent" role="listbox">
    <div id="lightElement" role="option">Hello world!</div>
  </div>
  <div id="shadowHostElement"></div>

  <script>
  test(function(t) {
    const lightElement = document.getElementById("lightElement");
    const shadowRoot = shadowHostElement.attachShadow({mode: "open"});

    assert_equals(lightParent.ariaActiveDescendantElement, null, 'null before');
    assert_equals(lightParent.getAttribute('aria-activedescendant'), null, 'null before');

    lightParent.ariaActiveDescendantElement = lightElement;
    assert_equals(lightParent.ariaActiveDescendantElement, lightElement);
    assert_equals(lightParent.getAttribute('aria-activedescendant'), "");

    // Move the referenced element into shadow DOM.
    // This will cause the computed attr-associated element to be null as the
    // referenced element will no longer be in a valid scope.
    // The underlying reference is kept intact, so if the referenced element is
    // later restored to a valid scope the computed attr-associated element will
    // then reflect
    shadowRoot.appendChild(lightElement);
    assert_equals(lightParent.ariaActiveDescendantElement, null, "computed attr-assoc element should be null as referenced element is in an invalid scope");
    assert_equals(lightParent.getAttribute("aria-activedescendant"), "");

    // Move the referenced element back into light DOM.
    // Since the underlying reference was kept intact, after moving the
    // referenced element back to a valid scope should be reflected in the
    // computed attr-associated element.
    lightParent.appendChild(lightElement);
    assert_equals(lightParent.ariaActiveDescendantElement, lightElement, "computed attr-assoc element should be restored as referenced element is back in a valid scope");
    assert_equals(lightParent.getAttribute("aria-activedescendant"), "");
  }, "Reparenting an element into a descendant shadow scope hides the element reference.");
  </script>

  <div id='fruitbowl' role='listbox'>
    <div id='apple' role='option'>I am an apple</div>
    <div id='pear' role='option'>I am a pear</div>
    <div id='banana' role='option'>I am a banana</div>
  </div>
  <div id='shadowFridge'></div>

  <script>
  test(function(t) {
    const shadowRoot = shadowFridge.attachShadow({mode: "open"});
    const banana = document.getElementById("banana");

    fruitbowl.ariaActiveDescendantElement = apple;
    assert_equals(fruitbowl.ariaActiveDescendantElement, apple);
    assert_equals(fruitbowl.getAttribute("aria-activedescendant"), "");

    // Move the referenced element into shadow DOM.
    shadowRoot.appendChild(apple);
    assert_equals(fruitbowl.ariaActiveDescendantElement, null, "computed attr-assoc element should be null as referenced element is in an invalid scope");
    // The content attribute is still empty.
    assert_equals(fruitbowl.getAttribute("aria-activedescendant"), "");

    // let us rename our banana to an apple
    banana.setAttribute("id", "apple");
    const lyingBanana = document.getElementById("apple");
    assert_equals(lyingBanana, banana);

    // our ariaActiveDescendantElement thankfully isn't tricked.
    // this is thanks to the underlying reference being kept intact, it is
    // checked and found to be in an invalid scope.
    assert_equals(fruitbowl.ariaActiveDescendantElement, null);
    // our content attribute is empty.
    assert_equals(fruitbowl.getAttribute("aria-activedescendant"), "");

    // when we remove our IDL attribute, the content attribute is also thankfully cleared.
    fruitbowl.ariaActiveDescendantElement = null;
    assert_equals(fruitbowl.ariaActiveDescendantElement, null);
    assert_equals(fruitbowl.getAttribute("aria-activedescendant"), null);
  }, "Reparenting referenced element cannot cause retargeting of reference.");
  </script>

  <div id='toaster' role='listbox'></div>
  <div id='shadowPantry'></div>

  <script>
  test(function(t) {
    const shadowRoot = shadowPantry.attachShadow({mode: "open"});

    // Our toast starts in the shadowPantry.
    const toast = document.createElement("div");
    toast.setAttribute("id", "toast");
    shadowRoot.appendChild(toast);

    // Prepare my toast for toasting
    toaster.ariaActiveDescendantElement = toast;
    assert_equals(toaster.ariaActiveDescendantElement, null);
    assert_equals(toaster.getAttribute("aria-activedescendant"), "");

    // Time to make some toast
    toaster.appendChild(toast);
    assert_equals(toaster.ariaActiveDescendantElement, toast);
    // Current spec behaviour:
    assert_equals(toaster.getAttribute("aria-activedescendant"), "");
  }, "Element reference set in invalid scope remains intact throughout move to valid scope.");
  </script>

  <div id="billingElementContainer">
      <div id="billingElement">Billing</div>
  </div>
  <div>
      <div id="nameElement">Name</div>
      <input type="text" id="input1" aria-labelledby="billingElement nameElement"/>
  </div>
  <div>
      <div id="addressElement">Address</div>
      <input type="text" id="input2"/>
  </div>

  <script>
  test(function(t) {
    const billingElement = document.getElementById("billingElement")
    assert_array_equals(input1.ariaLabelledByElements, [billingElement, nameElement], "parsed content attribute sets element references.");
    assert_equals(input1.ariaLabelledByElements, input1.ariaLabelledByElements, "check idl attribute caching after parsing");
    assert_e
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Aria",
      "code": "aria.activedescendant.descendant_or_owns",
      "message": "Attribute “aria-activedescendant” value should either refer to a descendant element, or should be accompanied by attribute “aria-owns”.",
      "severity": "Warning",
      "span": {
        "byte_end": 542,
        "byte_start": 489,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “multiple-id”.",
      "severity": "Error",
      "span": {
        "byte_end": 4160,
        "byte_start": 4124,
        "col": 5,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 6075,
        "byte_start": 6067,
        "col": 26,
        "line": 120
      }
    },
    {
      "category": "Aria",
      "code": "aria.li.role.descendant_of_tablist",
      "message": "An “li” element that is a descendant of a “role=tablist” element must not have any “role” value other than “tab”.",
      "severity": "Error",
      "span": {
        "byte_end": 18378,
        "byte_start": 18354,
        "col": 5,
        "line": 389
      }
    },
    {
      "category": "Aria",
      "code": "aria.li.role.descendant_of_tablist",
      "message": "An “li” element that is a descendant of a “role=tablist” element must not have any “role” value other than “tab”.",
      "severity": "Error",
      "span": {
        "byte_end": 18469,
        "byte_start": 18445,
        "col": 5,
        "line": 390
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 21156,
        "byte_start": 21146,
        "col": 34,
        "line": 455
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 21200,
        "byte_start": 21190,
        "col": 34,
        "line": 456
      }
    },
    {
      "category": "Aria",
      "code": "aria.aria_labelledby.prohibited_on_role.generic",
      "message": "The “aria-labelledby” attribute must not be specified on any “div” element unless the element has a “role” value other than “caption”, “code”, “deletion”, “emphasis”, “generic”, “insertion”, “paragraph”, “presentation”, “strong”, “subscript”, or “superscript”.",
      "severity": "Error",
      "span": {
        "byte_end": 28812,
        "byte_start": 28745,
        "col": 5,
        "line": 614
      }
    },
    {
      "category": "Aria",
      "code": "aria.idref.missing.aria-flowto",
      "message": "The “aria-flowto” attribute references “shadowChild1”, which is not the ID of any element in this document.",
      "severity": "Error",
      "span": {
        "byte_end": 23695,
        "byte_start": 23630,
        "col": 5,
        "line": 521
      }
    },
    {
      "category": "Aria",
      "code": "aria.idref.missing.aria-flowto",
      "message": "The “aria-flowto” attribute references “shadowChild2”, which is not the ID of any element in this document.",
      "severity": "Error",
      "span": {
        "byte_end": 23695,
        "byte_start": 23630,
        "col": 5,
        "line": 521
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/aria-element-reflection.html"
}
```
