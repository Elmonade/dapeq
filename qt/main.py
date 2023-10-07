import sys
import os
from PyQt6.QtCore import Qt, QSize
from PyQt6.QtGui import QIcon, QFont
from PyQt6.QtMultimedia import QMediaPlayer, QMediaPlaylist, QMediaContent
from PyQt6.QtWidgets import QApplication, QMainWindow, QWidget, QPushButton, QVBoxLayout, QLabel, QStackedWidget, QListWidget, QListWidgetItem, QScrollArea

class MusicListWindow(QWidget):
    def __init__(self, parent, music_directory):
        super().__init__()

        self.parent = parent  # Keep a reference to the parent window
        self.music_directory = music_directory

        # Layout
        layout = QVBoxLayout(self)
        layout.setAlignment(Qt.AlignmentFlag.AlignTop)

        # Scrollable List of Music Tracks
        scroll_area = QScrollArea(self)
        self.list_widget = QListWidget()
        scroll_area.setWidget(self.list_widget)
        layout.addWidget(scroll_area)

        # Back Button
        back_button = QPushButton("Back")
        back_button.clicked.connect(self.go_back)
        layout.addWidget(back_button)

        # Load music files
        self.load_music_files()

    def go_back(self):
        self.parent.stacked_widget.setCurrentIndex(0)  # Show the main window

    def load_music_files(self):
        # Clear the existing list
        self.list_widget.clear()

        # Iterate through the music directory and add music files to the list
        if os.path.exists(self.music_directory) and os.path.isdir(self.music_directory):
            for root, dirs, files in os.walk(self.music_directory):
                for file in files:
                    if file.endswith(('.mp3', '.wav', '.ogg', '.flac')):  # Add more file extensions as needed
                        self.add_music_track(file)

    def add_music_track(self, track_name):
        item = QListWidgetItem(track_name)
        self.list_widget.addItem(item)

class MusicPlayer(QMainWindow):
    def __init__(self):
        super().__init__()

        # Set window properties
        self.setWindowTitle("Minimalistic Music Player")
        self.setGeometry(100, 100, 128, 128)

        # Create a stacked widget
        self.stacked_widget = QStackedWidget(self)
        self.setCentralWidget(self.stacked_widget)

        # Main Window Widget
        main_window_widget = QWidget()
        layout = QVBoxLayout(main_window_widget)
        layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

        # Play/Pause Button with Icon
        self.playing = False
        self.play_pause_button = QPushButton()
        self.play_pause_button.setIcon(QIcon("play.png"))
        self.play_pause_button.setIconSize(QSize(48, 48))
        self.play_pause_button.clicked.connect(self.toggle_play_pause)
        layout.addWidget(self.play_pause_button)

        # Track Information Label with Terminus font
        self.track_label = QLabel("Now Playing: No Track")
        font = QFont("Terminus")
        font.setPointSize(8)
        self.track_label.setFont(font)
        self.track_label.setStyleSheet("color: green;")
        layout.addWidget(self.track_label)

        # Music List Button
        music_list_button = QPushButton("Music List")
        music_list_button.clicked.connect(self.show_music_list)
        layout.addWidget(music_list_button)

        # Add the main window widget to the stacked widget
        self.stacked_widget.addWidget(main_window_widget)

        # Keep track of the currently playing track
        self.currently_playing_track = None

        # Create a QMediaPlayer instance
        self.media_player = QMediaPlayer()
        self.media_player.stateChanged.connect(self.on_media_state_changed)

    def toggle_play_pause(self):
        if self.playing:
            self.play_pause_button.setIcon(QIcon("play.png"))
            self.playing = False
            self.media_player.pause()
        else:
            if self.currently_playing_track:
                self.play_pause_button.setIcon(QIcon("pause.png"))
                self.playing = True
                self.media_player.play()
            else:
                self.play_first_track()

    def play_first_track(self):
        if os.path.exists(self.music_directory) and os.path.isdir(self.music_directory):
            for root, dirs, files in os.walk(self.music_directory):
                for file in files:
                    if file.endswith(('.mp3', '.wav', '.ogg', '.flac')):
                        track_path = os.path.join(root, file)
                        self.currently_playing_track = track_path
                        playlist = QMediaPlaylist()
                        playlist.addMedia(QMediaContent(QUrl.fromLocalFile(track_path)))
                        self.media_player.setPlaylist(playlist)
                        self.play_pause_button.setIcon(QIcon("pause.png"))
                        self.playing = True
                        self.media_player.play()
                        self.update_track_label(track_path)
                        return

    def update_track_label(self, track_path):
        track_name = os.path.basename(track_path)
        self.track_label.setText(f"Now Playing: {track_name}")

    def on_media_state_changed(self, state):
        if state == QMediaPlayer.State.StoppedState:
            self.playing = False
            self.play_pause_button.setIcon(QIcon("play.png"))

    def show_music_list(self):
        music_list_window = MusicListWindow(self, self.music_directory)
        self.stacked_widget.addWidget(music_list_window)
        self.stacked_widget.setCurrentWidget(music_list_window)

def main():
    app = QApplication(sys.argv)
    player = MusicPlayer()
    player.setStyleSheet("background-color: white;")
    player.showFullScreen()  # Show the main window in fullscreen mode

    # Set the static path to your music directory here
    music_directory = "/home/jello/Media/Audio/"
    player.music_directory = music_directory

    sys.exit(app.exec())

if __name__ == "__main__":
    main()
