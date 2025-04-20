// fullscreenn pop up
let popup = document.getElementById("popup");
let images = document.getElementsByClassName("popImage");
let imgBig = document.getElementById("imgBig");
let subtext = document.getElementById("subtext");
let imgComment = document.getElementsByClassName("imgComment");
let exit = document.getElementById("x");
let arrows = document.getElementsByClassName("arrow");
let index = 0;



subtext.textContent = imgComment[index].textContent

exit.addEventListener("click", function () {
    popup.style.opacity = "0";
    popup.style.zIndex = "-1";
    document.body.style.overflow = "initial";
})



// opens popup
for (let i = 0; i < images.length; i++) {

    images[i].addEventListener("click", function () {
        window.scrollTo(0, 0);
        document.body.style.overflow = "hidden";
        popup.style.opacity = "1";
        popup.style.zIndex = "13";
        imgBig.src = images[i].src;
        setSubtext(i)
        index = i;
    })
}



// arrows
arrows[0].addEventListener("click", function () {
    leftArrow();
})
arrows[1].addEventListener("click", function () {
    index = index + 1;
    if (index > images.length - 1) {
        index = 0;
    }
    imgBig.src = images[index].src;
    setSubtext(index)
})



//  keypressevents
document.addEventListener('keydown', evt => {
    if (evt.key === 'Escape') {
        popup.style.opacity = "0";
        popup.style.zIndex = "-1";
        document.body.style.overflow = "initial";
    }
    if (evt.key === 'ArrowLeft') {
        leftArrow();
    }
    if (evt.key === 'ArrowRight') {
        index = index + 1;
        if (index > images.length - 1) {
            index = 0;
        }
        imgBig.src = images[index].src;
        setSubtext(index)
    }
});



function leftArrow() {
    index = index - 1;
    if (index < 0) {
        index = images.length - 1;
    }
    imgBig.src = images[index].src;
    setSubtext(index)
}



function setSubtext(index) {
    console.log(imgComment[index])
    if (imgComment[index].textContent !== "---") {
        subtext.textContent = imgComment[index].textContent;
    } else {
        subtext.textContent = ""
    }

}
