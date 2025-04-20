var menuItem = document.querySelector(".fp_sidebar .works_menu .menu_toggle");

menuItem.addEventListener("click", function(e) {
  e.preventDefault();
  var subMenu = this.nextElementSibling;
  if (this.parentNode.classList.contains("active")) {
    this.parentNode.classList.remove("active");
    subMenu.style.display = "none";
  } else {
    this.parentNode.classList.add("active");
    subMenu.style.display = "block";
  }
});

var subMenu = document.querySelector(".sidebar .works_menu .sub_menu");

if (!window.location.href.includes("project")) {
    subMenu.style.display = "none";
}
