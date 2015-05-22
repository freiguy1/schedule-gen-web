<!DOCTYPE html>
<html ng-app="ScheduleGenApp">
    <head>
    <meta name="viewport" content="initial-scale=1">
        <link rel="stylesheet" href="https://ajax.googleapis.com/ajax/libs/angular_material/0.8.3/angular-material.min.css">
        <title>
            League Schedule Generator
        </title>
    </head>
    <body ng-controller="AppCtrl">
        <md-content layout-padding>
            <md-toolbar>
                <div class="md-toolbar-tools">
                    <h2><span>Teams</span></h2>
                    <span flex></span>
                    <md-button ng-click="addTeam()" class="md-icon-button">
                        <md-icon md-svg-icon="assets/images/plus.svg" style="color: greenyellow; height:40px; width:40px"></md-icon>
                    </md-button>
                </div>
            </md-toolbar>

            <md-input-container ng-repeat="teamName in teams track by $index">
                <label>Team Name</label>
                <input ng-model="teamName">
            </md-input-container>
        <md-content>
        <script type="text/javascript" src="http://code.jquery.com/jquery-2.1.4.min.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/angularjs/1.3.15/angular.min.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/angularjs/1.3.15/angular-animate.min.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/angularjs/1.3.15/angular-aria.min.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/angular_material/0.8.3/angular-material.min.js"></script>
        <script type="text/javascript" src="/assets/js.js"></script>
    </body>
</html>
