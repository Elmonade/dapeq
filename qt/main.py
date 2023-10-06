from PyQt6 import QtWidgets as qtw


class MainWindow(qtw.QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('Lemon')
        self.setLayout(qtw.QVBoxLayout())
        self.search()

        self.show()

    def search(self):
        container = qtw.QWidget()
        container.setLayout(qtw.QGridLayout())

        self.searchField = qtw.QLineEdit()
        buttonSearch = qtw.QPushButton(
                'SEARCH', clicked=lambda: self.searchField.setText('Test'))

        container.layout().addWidget(self.searchField, 0, 0, 1, 5)
        container.layout().addWidget(buttonSearch, 5, 3)

        self.layout().addWidget(container)


app = qtw.QApplication([])

mw = MainWindow()
app.setStyle(qtw.QStyleFactory.create('Fusion'))
app.exec()
