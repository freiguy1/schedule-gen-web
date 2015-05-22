var app = angular.module('ScheduleGenApp', ['ngMaterial']);


app.controller('AppCtrl', function ($scope) {
    $scope.teams = [ "Team 1", "Team 2" ];
    $scope.addTeam = function() { $scope.teams.push(''); };
});
